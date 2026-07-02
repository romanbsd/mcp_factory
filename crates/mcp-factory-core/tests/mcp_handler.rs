mod common;

use mcp_factory_core::{AuthConfig, McpProxyServer, ProxyConfig};
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn forwards_bearer_token() {
    temp_env::with_var("MCP_FACTORY_BEARER_TOKEN", Some("secret-token"), || async {
        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/pets/1"))
            .and(header("authorization", "Bearer secret-token"))
            .respond_with(ResponseTemplate::new(200).set_body_string("{}"))
            .mount(&mock_server)
            .await;

        let mut config = common::proxy_config(&mock_server.uri());
        config.auth = AuthConfig::bearer();
        let server = McpProxyServer::builder(config)
            .tools(&[common::rest_get_pet_tool()])
            .unwrap()
            .build()
            .unwrap();

        server
            .invoke_tool("get_pet", json!({"petId": 1}))
            .await
            .unwrap();
    })
    .await;
}

#[tokio::test]
async fn lists_and_reads_resources() {
    let config = ProxyConfig::default();
    let resources = common::sample_resources();
    let server = McpProxyServer::builder(config)
        .resources(&resources)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(server.tool_count(), 0);
    let content = server.read_resource_content("schema://openapi").unwrap();
    assert!(content.contains("openapi"));
}

#[tokio::test]
async fn rejects_invalid_tool_args() {
    let mock_server = MockServer::start().await;
    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    assert!(server.invoke_tool("get_pet", json!({})).await.is_err());
}

#[tokio::test]
async fn registers_multiple_tools() {
    let config = ProxyConfig::default();
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool(), common::rest_create_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    let mut names = server.tool_names();
    names.sort();
    assert_eq!(names, vec!["create_pet", "get_pet"]);
}

mod temp_env {
    use std::env;

    pub async fn with_var<F, Fut>(key: &str, value: Option<&str>, f: F) -> Fut::Output
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let previous = env::var(key).ok();
        match value {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
        let out = f().await;
        match previous {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
        out
    }
}
