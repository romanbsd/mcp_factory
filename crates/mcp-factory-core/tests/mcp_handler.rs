mod common;

use std::sync::Arc;

use mcp_factory_core::{
    async_trait, AuthConfig, CustomToolHandler, CustomToolSpec, McpProxyServer, ProxyConfig,
    ProxyError, ReadOnlyToolInvoker, ToolHints, ToolResult,
};
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

struct PetSummaryHandler {
    source_tool: &'static str,
}

#[async_trait]
impl CustomToolHandler for PetSummaryHandler {
    async fn call(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        arguments: serde_json::Value,
    ) -> Result<ToolResult, ProxyError> {
        let source = invoker
            .invoke_read_only(self.source_tool, arguments)
            .await?;
        let pet = source
            .structured
            .ok_or_else(|| ProxyError::Other("source returned no JSON".to_string()))?;
        let name = pet["name"]
            .as_str()
            .ok_or_else(|| ProxyError::Other("source returned no pet name".to_string()))?;
        let structured = json!({"summary": format!("pet: {name}")});
        Ok(ToolResult::text(structured.to_string()).with_structured(Some(structured)))
    }
}

fn custom_pet_summary(source_tool: &'static str) -> CustomToolSpec {
    CustomToolSpec {
        name: "report_pet".to_string(),
        description: "Summarize a pet".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {"petId": {"type": "integer"}},
            "required": ["petId"]
        }),
        hints: ToolHints {
            read_only: Some(true),
            destructive: Some(false),
            idempotent: Some(true),
            ..Default::default()
        },
        handler: Arc::new(PetSummaryHandler { source_tool }),
    }
}

#[tokio::test]
async fn custom_tool_invokes_and_transforms_read_only_generated_tool() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/pets/7"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "Mochi"})))
        .mount(&mock_server)
        .await;

    let mut source = common::rest_get_pet_tool();
    source.hints.read_only = Some(true);
    let server = McpProxyServer::builder(common::proxy_config(&mock_server.uri()))
        .tools(&[source])
        .unwrap()
        .custom_tools(&[custom_pet_summary("get_pet")])
        .unwrap()
        .build()
        .unwrap();

    let mut names = server.tool_names();
    names.sort();
    assert_eq!(names, ["get_pet", "report_pet"]);
    assert_eq!(server.tool_count(), 2);
    assert_eq!(
        server
            .invoke_tool("report_pet", json!({"petId": 7}))
            .await
            .unwrap(),
        r#"{"summary":"pet: Mochi"}"#
    );
    assert!(server
        .invoke_tool("report_pet", json!({"petId": "wrong"}))
        .await
        .unwrap_err()
        .to_string()
        .contains("invalid arguments for report_pet"));
}

#[tokio::test]
async fn custom_tool_cannot_invoke_generated_mutation() {
    let server = McpProxyServer::builder(ProxyConfig::default())
        .tools(&[common::rest_create_pet_tool()])
        .unwrap()
        .custom_tools(&[custom_pet_summary("create_pet")])
        .unwrap()
        .build()
        .unwrap();

    let error = server
        .invoke_tool("report_pet", json!({"petId": 7}))
        .await
        .unwrap_err();
    assert_eq!(
        error.to_string(),
        "validation error: custom tools may invoke only generated read-only tools: create_pet"
    );
}

#[test]
fn rejects_duplicate_names_across_generated_and_custom_tools() {
    let mut generated = common::rest_get_pet_tool();
    generated.name = "report_pet".to_string();
    let custom = custom_pet_summary("get_pet");

    let generated_first = McpProxyServer::builder(ProxyConfig::default())
        .tools(std::slice::from_ref(&generated))
        .unwrap()
        .custom_tools(std::slice::from_ref(&custom));
    let generated_first = match generated_first {
        Ok(_) => panic!("duplicate generated/custom name was accepted"),
        Err(error) => error,
    };
    assert_eq!(
        generated_first.to_string(),
        "duplicate tool name: report_pet"
    );

    let custom_first = McpProxyServer::builder(ProxyConfig::default())
        .custom_tools(&[custom])
        .unwrap()
        .tools(&[generated]);
    let custom_first = match custom_first {
        Ok(_) => panic!("duplicate custom/generated name was accepted"),
        Err(error) => error,
    };
    assert_eq!(custom_first.to_string(), "duplicate tool name: report_pet");
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
