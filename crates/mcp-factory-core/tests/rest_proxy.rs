mod common;

use mcp_factory_core::McpProxyServer;
use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn rest_proxy_gets_pet_by_id() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/pets/42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 42, "name": "fluffy"})))
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    let result = server
        .invoke_tool("get_pet", json!({"petId": 42}))
        .await
        .unwrap();

    assert!(result.contains("fluffy"));
}

#[tokio::test]
async fn rest_proxy_posts_json_body() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/pets"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"id": 1})))
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_create_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    server
        .invoke_tool("create_pet", json!({"name": "fluffy", "tag": "dog"}))
        .await
        .unwrap();
}
