mod common;

use mcp_factory_core::McpProxyServer;
use serde_json::json;
use wiremock::matchers::{body_partial_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn graphql_proxy_posts_query() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/"))
        .and(body_partial_json(json!({
            "query": "query($id: ID!) { user(id: $id) { name } }",
            "variables": {"id": "1"}
        })))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(json!({"data": {"user": {"name": "alice"}}})),
        )
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[common::graphql_user_tool()])
        .unwrap()
        .build()
        .unwrap();

    let result = server
        .invoke_tool("get_user", json!({"id": "1"}))
        .await
        .unwrap();

    assert!(result.contains("alice"));
}
