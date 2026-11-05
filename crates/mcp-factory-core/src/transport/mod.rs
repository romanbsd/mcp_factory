use std::sync::Arc;

use rmcp::ServiceExt;

use crate::config::TransportMode;
use crate::error::ProxyError;
use crate::server::McpProxyServer;

pub async fn run(server: McpProxyServer) -> Result<(), ProxyError> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_writer(std::io::stderr)
        .init();

    match server.config().transport {
        TransportMode::Stdio => run_stdio(server).await,
        TransportMode::Http => run_http(server).await,
        // Serve both transports concurrently; whichever finishes first — stdin
        // EOF (client disconnect) or an error on either — ends the process.
        TransportMode::Both => {
            tokio::select! {
                result = run_stdio(server.clone()) => result,
                result = run_http(server) => result,
            }
        }
    }
}

async fn run_stdio(server: McpProxyServer) -> Result<(), ProxyError> {
    let service = server
        .serve(rmcp::transport::stdio())
        .await
        .map_err(|e| ProxyError::Transport(e.to_string()))?;
    service
        .waiting()
        .await
        .map_err(|e| ProxyError::Transport(e.to_string()))?;
    Ok(())
}

async fn run_http(server: McpProxyServer) -> Result<(), ProxyError> {
    use std::net::SocketAddr;

    let config = server.config().clone();
    let bind_addr: SocketAddr = config
        .bind_addr
        .parse()
        .map_err(|e| ProxyError::Config(format!("invalid bind address: {e}")))?;
    let cancellation = tokio_util::sync::CancellationToken::new();
    let app = http_router(server, cancellation.clone());

    tracing::info!(%bind_addr, path = %config.http_path, "starting MCP HTTP transport");

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|e| ProxyError::Transport(e.to_string()))?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            cancellation.cancelled_owned().await;
        })
        .await
        .map_err(|e| ProxyError::Transport(e.to_string()))?;
    Ok(())
}

/// Build the transport without binding a socket. Identity providers own user
/// policy; core only isolates credentials and sessions by validated principal.
pub fn http_router(
    server: McpProxyServer,
    cancellation: tokio_util::sync::CancellationToken,
) -> axum::Router {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        response::{IntoResponse, Response},
        Router,
    };
    use rmcp::transport::streamable_http_server::{
        session::{local::LocalSessionManager, never::NeverSessionManager},
        tower::{StreamableHttpServerConfig, StreamableHttpService},
    };
    use std::collections::HashMap;
    use tower::ServiceExt;
    let path = server.config().http_path.clone();
    enum Services {
        Stateless(Router),
        Stateful(tokio::sync::Mutex<HashMap<String, Router>>),
    }
    // Stateless requests carry their own identity in HTTP Parts. They share a
    // service and never allocate or lock a per-principal session map.
    let services = Arc::new(if server.config().http_stateless {
        let server = server.clone();
        let config = StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_cancellation_token(cancellation.clone());
        let service = StreamableHttpService::new(
            move || Ok(server.clone()),
            Arc::new(NeverSessionManager::default()),
            config,
        );
        Services::Stateless(Router::new().fallback_service(service))
    } else {
        Services::Stateful(tokio::sync::Mutex::new(HashMap::new()))
    });
    let handler = tower::service_fn(move |mut request: Request<Body>| {
        let (server, cancellation, services) =
            (server.clone(), cancellation.clone(), services.clone());
        async move {
            let principal = if server.http_identity_required() {
                let values = request.headers().get_all(axum::http::header::AUTHORIZATION);
                let mut values = values.iter();
                let bearer = values.next().and_then(|v| v.to_str().ok()).and_then(|v| {
                    let (scheme, token) = v.split_once(' ')?;
                    (scheme.eq_ignore_ascii_case("bearer")
                        && !token.is_empty()
                        && token.len() <= 8192
                        && !token.bytes().any(|b| b.is_ascii_whitespace()))
                    .then_some(token)
                });
                let identity = if values.next().is_none() {
                    match bearer {
                        Some(token) => server.authenticate_http(token).await.ok(),
                        None => None,
                    }
                } else {
                    None
                };
                let Some(identity) =
                    identity.filter(|i| !i.principal.is_empty() && i.principal.len() <= 256)
                else {
                    let response = (
                        StatusCode::UNAUTHORIZED,
                        [("www-authenticate", "Bearer realm=\"mcp\"")],
                        "MCP authentication required",
                    )
                        .into_response();
                    return Ok::<Response, std::convert::Infallible>(response);
                };
                let principal = identity.principal.clone();
                request
                    .headers_mut()
                    .remove(axum::http::header::AUTHORIZATION);
                request.extensions_mut().insert(identity);
                principal
            } else {
                String::new()
            };
            let service = match services.as_ref() {
                Services::Stateless(service) => service.clone(),
                Services::Stateful(services) => {
                    let mut services = services.lock().await;
                    if !services.contains_key(&principal) {
                        // Bound process state even if a defective adapter invents identities.
                        if services.len() >= 1024 {
                            return Ok(StatusCode::SERVICE_UNAVAILABLE.into_response());
                        }
                        let config = StreamableHttpServerConfig::default()
                            .with_legacy_session_mode(true)
                            .with_cancellation_token(cancellation);
                        let service = StreamableHttpService::new(
                            move || Ok(server.clone()),
                            Arc::new(LocalSessionManager::default()),
                            config,
                        );
                        services.insert(principal.clone(), Router::new().fallback_service(service));
                    }
                    services[&principal].clone()
                }
            };
            service.oneshot(request).await
        }
    });
    Router::new().nest_service(&path, handler)
}
