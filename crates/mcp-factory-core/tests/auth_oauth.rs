mod common;

use chrono::{Duration, Utc};
use mcp_factory_core::auth::{oauth_provider_from_config, FileTokenStore, StoredTokens};
use mcp_factory_core::{AuthConfig, McpProxyServer, ProxyConfig};
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn oauth_auth_config(mock: &MockServer, token_store: std::path::PathBuf) -> AuthConfig {
    AuthConfig::OAuth2 {
        authorization_endpoint: format!("{}/oauth/authorize", mock.uri()),
        token_endpoint: format!("{}/oauth/token", mock.uri()),
        client_id: "test-client".to_string(),
        client_secret_env: "MCP_FACTORY_OAUTH_CLIENT_SECRET".to_string(),
        scopes: vec!["read".to_string()],
        redirect_uri: None,
        token_store,
    }
}

#[tokio::test]
async fn refreshes_expired_access_token() {
    let mock_server = MockServer::start().await;
    let token_dir = tempfile::tempdir().unwrap();
    let token_path = token_dir.path().join("tokens.json");
    let auth = oauth_auth_config(&mock_server, token_path.clone());

    let store = FileTokenStore::new(token_path);
    store
        .save(&StoredTokens {
            access_token: "old-access".to_string(),
            refresh_token: Some("refresh-abc".to_string()),
            expires_at: Some(Utc::now() - Duration::minutes(1)),
            token_type: "Bearer".to_string(),
        })
        .unwrap();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .and(body_string_contains("grant_type=refresh_token"))
        .and(body_string_contains("refresh_token=refresh-abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "new-access",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "refresh-abc"
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/pets/1"))
        .and(header("authorization", "Bearer new-access"))
        .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
        .mount(&mock_server)
        .await;

    let mut config = common::proxy_config(&mock_server.uri());
    config.auth = auth;
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    server
        .invoke_tool("get_pet", json!({"petId": 1}))
        .await
        .unwrap();
}

#[tokio::test]
async fn exchange_code_persists_tokens() {
    let mock_server = MockServer::start().await;
    let token_dir = tempfile::tempdir().unwrap();
    let token_path = token_dir.path().join("tokens.json");
    let auth = oauth_auth_config(&mock_server, token_path.clone());

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .and(body_string_contains("grant_type=authorization_code"))
        .and(body_string_contains("code=auth-code-123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "fresh-access",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "fresh-refresh"
        })))
        .mount(&mock_server)
        .await;

    let http = reqwest::Client::new();
    let provider = oauth_provider_from_config(&auth, http).unwrap();
    let (_challenge, verifier) = oauth2::PkceCodeChallenge::new_random_sha256();
    let redirect = "http://127.0.0.1:9876/callback";

    let tokens = provider
        .exchange_code("auth-code-123", redirect, verifier)
        .await
        .unwrap();
    provider.persist(&tokens).await.unwrap();

    let loaded = FileTokenStore::new(token_path).load().unwrap().unwrap();
    assert_eq!(loaded.access_token, "fresh-access");
    assert_eq!(loaded.refresh_token, Some("fresh-refresh".to_string()));
}

#[tokio::test]
async fn unauthenticated_oauth_returns_clear_error() {
    let mock_server = MockServer::start().await;
    let token_dir = tempfile::tempdir().unwrap();
    let token_path = token_dir.path().join("tokens.json");
    let mut config = common::proxy_config(&mock_server.uri());
    config.auth = oauth_auth_config(&mock_server, token_path);

    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    let err = server
        .invoke_tool("get_pet", json!({"petId": 1}))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("OAuth not authenticated"));
}

#[tokio::test]
async fn refresh_failure_surfaces_error() {
    let mock_server = MockServer::start().await;
    let token_dir = tempfile::tempdir().unwrap();
    let token_path = token_dir.path().join("tokens.json");
    let auth = oauth_auth_config(&mock_server, token_path.clone());

    FileTokenStore::new(token_path)
        .save(&StoredTokens {
            access_token: "expired".to_string(),
            refresh_token: Some("bad-refresh".to_string()),
            expires_at: Some(Utc::now() - Duration::minutes(1)),
            token_type: "Bearer".to_string(),
        })
        .unwrap();

    Mock::given(method("POST"))
        .and(path("/oauth/token"))
        .respond_with(ResponseTemplate::new(401).set_body_string("invalid_grant"))
        .mount(&mock_server)
        .await;

    let http = reqwest::Client::new();
    let provider = oauth_provider_from_config(&auth, http).unwrap();
    let err = provider.bearer_token().await.unwrap_err();
    assert!(err.to_string().contains("OAuth refresh failed"));
}
