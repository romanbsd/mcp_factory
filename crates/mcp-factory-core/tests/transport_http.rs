use std::sync::Arc;
use std::time::Duration;

use mcp_factory_core::{McpProxyServer, ProxyConfig, TransportMode};
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::tower::{
    StreamableHttpServerConfig, StreamableHttpService,
};
use tokio_util::sync::CancellationToken;

mod common;

#[tokio::test]
async fn streamable_http_endpoint_accepts_initialize() {
    let config = common::proxy_config("http://127.0.0.1:9");
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    let cancellation = CancellationToken::new();
    let http_config = StreamableHttpServerConfig::default()
        .with_stateful_mode(true)
        .with_cancellation_token(cancellation.clone());
    let service = StreamableHttpService::new(
        move || Ok(server.clone()),
        Arc::new(LocalSessionManager::default()),
        http_config,
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let ct = cancellation.clone();
    tokio::spawn(async move {
        let app = axum::Router::new().nest_service("/mcp", service);
        let _ = axum::serve(listener, app)
            .with_graceful_shutdown(async move { ct.cancelled_owned().await })
            .await;
    });

    tokio::time::sleep(Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let body = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}"#;
    let response = client
        .post(format!("http://{addr}/mcp"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json, text/event-stream")
        .body(body)
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    cancellation.cancel();
}
