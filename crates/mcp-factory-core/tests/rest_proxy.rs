mod common;

use mcp_factory_core::{ExecutionKind, McpProxyServer, RestOperation, ToolSpec};
use serde_json::json;
use wiremock::matchers::{body_string_contains, header, method, path};
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

fn form_login_tool() -> ToolSpec {
    ToolSpec {
        name: "login".to_string(),
        description: "Form login".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {"user": {"type": "string"}, "pass": {"type": "string"}},
            "required": ["user", "pass"]
        }),
        execution: ExecutionKind::Rest(RestOperation {
            method: "POST".to_string(),
            path_template: "/login".to_string(),
            params: vec![],
            body_fields: vec!["user".to_string(), "pass".to_string()],
            content_type: Some("application/x-www-form-urlencoded".to_string()),
            raw_body: false,
        }),
        hints: Default::default(),
    }
}

#[tokio::test]
async fn rest_proxy_sends_form_urlencoded_body() {
    let mock_server = MockServer::start().await;
    // Matches only if the body is urlencoded (not JSON) with both fields.
    Mock::given(method("POST"))
        .and(path("/login"))
        .and(header(
            "content-type",
            "application/x-www-form-urlencoded",
        ))
        .and(body_string_contains("user=bob"))
        .and(body_string_contains("pass=secret"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[form_login_tool()])
        .unwrap()
        .build()
        .unwrap();

    let result = server
        .invoke_tool("login", json!({"user": "bob", "pass": "secret"}))
        .await
        .unwrap();
    assert_eq!(result, "ok");
}

fn binary_tool() -> ToolSpec {
    ToolSpec {
        name: "get_image".to_string(),
        description: "Fetch an image".to_string(),
        input_schema: json!({"type": "object", "properties": {}}),
        execution: ExecutionKind::Rest(RestOperation {
            method: "GET".to_string(),
            path_template: "/image".to_string(),
            params: vec![],
            body_fields: vec![],
            content_type: None,
            raw_body: false,
        }),
        hints: Default::default(),
    }
}

#[tokio::test]
async fn rest_proxy_base64s_binary_response() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/image"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "image/png")
                .set_body_bytes(vec![1u8, 2, 3]),
        )
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[binary_tool()])
        .unwrap()
        .build()
        .unwrap();

    // invoke_tool flattens binary output to base64; [1,2,3] -> "AQID".
    let result = server.invoke_tool("get_image", json!({})).await.unwrap();
    assert_eq!(result, "AQID");
}

#[tokio::test]
async fn rest_proxy_surfaces_upstream_error() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/pets/9"))
        .respond_with(ResponseTemplate::new(500).set_body_string("boom"))
        .mount(&mock_server)
        .await;

    let config = common::proxy_config(&mock_server.uri());
    let server = McpProxyServer::builder(config)
        .tools(&[common::rest_get_pet_tool()])
        .unwrap()
        .build()
        .unwrap();

    // Upstream 5xx surfaces as a tool error through invoke_tool.
    let err = server
        .invoke_tool("get_pet", json!({"petId": 9}))
        .await
        .unwrap_err();
    assert!(err.to_string().contains("500"));
}
