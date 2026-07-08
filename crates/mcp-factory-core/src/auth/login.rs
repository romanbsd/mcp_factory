use std::sync::Arc;
use std::time::Duration;

use axum::extract::Query;
use axum::routing::get;
use axum::Router;
use oauth2::{CsrfToken, PkceCodeChallenge};
use serde::Deserialize;
use tokio::sync::{oneshot, Mutex};

use crate::auth::oauth2::OAuth2Provider;
use crate::auth::provider::oauth_provider_from_config;
use crate::auth::token_store::StoredTokens;
use crate::config::{AuthConfig, ProxyConfig};
use crate::error::ProxyError;

const LOGIN_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Debug, Deserialize)]
struct CallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

/// Run interactive OAuth2 Authorization Code + PKCE login and persist the tokens.
pub async fn run_oauth_login(config: &ProxyConfig) -> Result<(), ProxyError> {
    let AuthConfig::OAuth2 { .. } = &config.auth else {
        return Err(ProxyError::Config(
            "auth.type must be oauth2 for login".to_string(),
        ));
    };

    let http = reqwest::Client::builder()
        .timeout(config.timeout())
        .build()?;
    let provider = oauth_provider_from_config(&config.auth, http)?;

    let tokens = perform_login(&provider).await?;
    provider.persist(&tokens).await?;
    eprintln!("OAuth login successful; tokens saved.");
    Ok(())
}

/// Drive the interactive browser flow and return the resulting tokens (without
/// persisting them). Shared by the explicit `login` command and the lazy,
/// on-demand login triggered from a tool call.
///
/// All progress goes to stderr: on a stdio MCP server, stdout is the JSON-RPC
/// channel and must not be written to.
pub(crate) async fn perform_login(provider: &OAuth2Provider) -> Result<StoredTokens, ProxyError> {
    let AuthConfig::OAuth2 {
        scopes,
        redirect_uri,
        ..
    } = provider.auth_config()
    else {
        return Err(ProxyError::Config(
            "auth.type must be oauth2 for login".to_string(),
        ));
    };

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let csrf_state = CsrfToken::new_random();
    let (redirect, listener) = prepare_callback(redirect_uri.as_deref()).await?;
    let auth_url = build_auth_url(
        provider.auth_config(),
        &redirect,
        scopes,
        pkce_challenge,
        &csrf_state,
    )?;

    eprintln!("Open this URL to authorize:\n{auth_url}");
    if open::that(&auth_url).is_err() {
        eprintln!("(Could not open browser automatically; copy the URL above.)");
    }

    let code = wait_for_callback(listener, &redirect, csrf_state.secret()).await?;
    provider
        .exchange_code(&code, &redirect, pkce_verifier)
        .await
}

fn build_auth_url(
    auth: &AuthConfig,
    redirect_uri: &str,
    scopes: &[String],
    pkce_challenge: PkceCodeChallenge,
    csrf_state: &CsrfToken,
) -> Result<String, ProxyError> {
    let AuthConfig::OAuth2 {
        authorization_endpoint,
        client_id,
        ..
    } = auth
    else {
        return Err(ProxyError::Config("expected OAuth2 config".to_string()));
    };

    let mut url = oauth2::url::Url::parse(authorization_endpoint)
        .map_err(|e| ProxyError::Config(format!("invalid authorization endpoint: {e}")))?;
    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("response_type", "code");
        pairs.append_pair("client_id", client_id);
        pairs.append_pair("redirect_uri", redirect_uri);
        pairs.append_pair("state", csrf_state.secret());
        pairs.append_pair("code_challenge", pkce_challenge.as_str());
        pairs.append_pair("code_challenge_method", "S256");
        if !scopes.is_empty() {
            pairs.append_pair("scope", &scopes.join(" "));
        }
    }
    Ok(url.to_string())
}

async fn prepare_callback(
    configured: Option<&str>,
) -> Result<(String, tokio::net::TcpListener), ProxyError> {
    if let Some(uri) = configured {
        let parsed = oauth2::url::Url::parse(uri)
            .map_err(|e| ProxyError::Config(format!("invalid redirect_uri: {e}")))?;
        let port = parsed.port_or_known_default().ok_or_else(|| {
            ProxyError::Config("redirect_uri must include an explicit port".to_string())
        })?;
        let bind_addr = format!("127.0.0.1:{port}");
        let listener = tokio::net::TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| ProxyError::Config(format!("failed to bind {bind_addr}: {e}")))?;
        return Ok((uri.to_string(), listener));
    }

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| ProxyError::Config(format!("failed to bind callback port: {e}")))?;
    let port = listener
        .local_addr()
        .map_err(|e| ProxyError::Config(format!("failed to read callback port: {e}")))?
        .port();
    Ok((format!("http://127.0.0.1:{port}/callback"), listener))
}

async fn wait_for_callback(
    listener: tokio::net::TcpListener,
    redirect_uri: &str,
    expected_state: &str,
) -> Result<String, ProxyError> {
    let path = oauth2::url::Url::parse(redirect_uri)
        .map_err(|e| ProxyError::Config(format!("invalid redirect_uri: {e}")))?
        .path()
        .to_string();

    let (tx, rx) = oneshot::channel::<Result<String, ProxyError>>();
    let tx = Arc::new(Mutex::new(Some(tx)));
    let expected_state = expected_state.to_string();

    let app = Router::new().route(
        &path,
        get({
            let tx = Arc::clone(&tx);
            let expected_state = expected_state.clone();
            move |Query(query): Query<CallbackQuery>| {
                let tx = Arc::clone(&tx);
                let expected_state = expected_state.clone();
                async move {
                    let mut guard = tx.lock().await;
                    if let Some(sender) = guard.take() {
                        let result = if let Some(err) = query.error {
                            let detail = query.error_description.unwrap_or_default();
                            Err(ProxyError::Config(format!(
                                "OAuth authorization error: {err} {detail}"
                            )))
                        } else if query.state.as_deref() != Some(expected_state.as_str()) {
                            Err(ProxyError::Config(
                                "OAuth callback state mismatch".to_string(),
                            ))
                        } else if let Some(code) = query.code {
                            Ok(code)
                        } else {
                            Err(ProxyError::Config(
                                "OAuth callback missing code".to_string(),
                            ))
                        };
                        let _ = sender.send(result);
                    }
                    "You can close this window."
                }
            }
        }),
    );

    let server = axum::serve(listener, app);
    let server_handle = tokio::spawn(async move {
        let _ = server.await;
    });

    let result = tokio::time::timeout(LOGIN_TIMEOUT, rx)
        .await
        .map_err(|_| ProxyError::Config("OAuth login timed out".to_string()))?
        .map_err(|_| ProxyError::Config("OAuth callback channel closed".to_string()))??;

    server_handle.abort();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn oauth_config() -> AuthConfig {
        AuthConfig::OAuth2 {
            authorization_endpoint: "https://auth.example.com/authorize".to_string(),
            token_endpoint: "https://auth.example.com/token".to_string(),
            client_id: "client".to_string(),
            client_secret_env: "SECRET".to_string(),
            scopes: vec!["read".to_string()],
            redirect_uri: None,
            token_store: "tokens.json".into(),
        }
    }

    #[test]
    fn auth_url_includes_csrf_state() {
        let (challenge, _verifier) = PkceCodeChallenge::new_random_sha256();
        let state = CsrfToken::new("state-123".to_string());
        let url = build_auth_url(
            &oauth_config(),
            "http://127.0.0.1:1234/callback",
            &["read".to_string()],
            challenge,
            &state,
        )
        .unwrap();
        let parsed = oauth2::url::Url::parse(&url).unwrap();
        let query = parsed
            .query_pairs()
            .collect::<std::collections::HashMap<_, _>>();
        assert_eq!(query.get("state").map(|v| v.as_ref()), Some("state-123"));
    }
}

pub async fn oauth_status(config: &ProxyConfig) -> Result<(), ProxyError> {
    let AuthConfig::OAuth2 { token_store, .. } = &config.auth else {
        return Err(ProxyError::Config("auth.type must be oauth2".to_string()));
    };
    let store = crate::auth::token_store::FileTokenStore::new(token_store.clone());
    match store.load()? {
        None => println!("No stored OAuth tokens."),
        Some(tokens) => {
            println!("Access token: present");
            println!(
                "Refresh token: {}",
                if tokens.refresh_token.is_some() {
                    "present"
                } else {
                    "absent"
                }
            );
            match tokens.expires_at {
                Some(exp) => println!("Expires at: {exp}"),
                None => println!("Expires at: unknown"),
            }
        }
    }
    Ok(())
}

pub async fn oauth_logout(config: &ProxyConfig) -> Result<(), ProxyError> {
    let AuthConfig::OAuth2 { token_store, .. } = &config.auth else {
        return Err(ProxyError::Config("auth.type must be oauth2".to_string()));
    };
    let store = crate::auth::token_store::FileTokenStore::new(token_store.clone());
    store.delete()?;
    println!("OAuth tokens deleted.");
    Ok(())
}
