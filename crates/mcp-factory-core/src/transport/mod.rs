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
        TransportMode::Both => {
            tracing::warn!(
                "MCP_TRANSPORT=both is not supported concurrently; falling back to stdio"
            );
            run_stdio(server).await
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

    use axum::Router;
    use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
    use rmcp::transport::streamable_http_server::tower::{
        StreamableHttpServerConfig, StreamableHttpService,
    };
    use tokio_util::sync::CancellationToken;

    let config = server.config().clone();
    let bind_addr: SocketAddr = config
        .bind_addr
        .parse()
        .map_err(|e| ProxyError::Config(format!("invalid bind address: {e}")))?;

    let session_manager = Arc::new(LocalSessionManager::default());
    let cancellation = CancellationToken::new();
    let http_config = StreamableHttpServerConfig::default()
        .with_stateful_mode(true)
        .with_cancellation_token(cancellation.clone());
    let path = config.http_path.clone();

    let service = StreamableHttpService::new(
        move || Ok(server.clone()),
        session_manager,
        http_config,
    );

    let app = Router::new().nest_service(&path, service);

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
