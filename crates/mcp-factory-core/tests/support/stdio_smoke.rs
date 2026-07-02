use mcp_factory_core::{
    ExecutionKind, McpProxyServer, ParamBinding, ParamLocation, ProxyConfig, RestOperation,
    ToolSpec, TransportMode,
};
use serde_json::json;

fn smoke_tool() -> ToolSpec {
    ToolSpec {
        name: "get_pet".to_string(),
        description: "Smoke-test tool".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "petId": { "type": "integer" }
            },
            "required": ["petId"]
        }),
        execution: ExecutionKind::Rest(RestOperation {
            method: "GET".to_string(),
            path_template: "/pets/{petId}".to_string(),
            params: vec![ParamBinding {
                name: "petId".to_string(),
                location: ParamLocation::Path,
            }],
            body_fields: vec![],
            content_type: None,
        }),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ProxyConfig {
        base_url: "http://127.0.0.1:1".to_string(),
        server_name: "stdio-smoke".to_string(),
        transport: TransportMode::Stdio,
        ..Default::default()
    };
    let tools = vec![smoke_tool()];
    let server = McpProxyServer::builder(config)
        .tools(&tools)?
        .build()?;
    server.run().await?;
    Ok(())
}
