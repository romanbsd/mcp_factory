use mcp_factory_core::{
    AuthConfig, ExecutionKind, ParamBinding, ParamLocation, ProxyConfig, ResourceSpec,
    RestOperation, ToolSpec,
};
use serde_json::json;

pub fn rest_get_pet_tool() -> ToolSpec {
    ToolSpec {
        name: "get_pet".to_string(),
        description: "Get pet by ID".to_string(),
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

pub fn rest_create_pet_tool() -> ToolSpec {
    ToolSpec {
        name: "create_pet".to_string(),
        description: "Create a pet".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "tag": { "type": "string" }
            },
            "required": ["name"]
        }),
        execution: ExecutionKind::Rest(RestOperation {
            method: "POST".to_string(),
            path_template: "/pets".to_string(),
            params: vec![],
            body_fields: vec!["name".to_string(), "tag".to_string()],
            content_type: Some("application/json".to_string()),
        }),
    }
}

pub fn graphql_user_tool() -> ToolSpec {
    ToolSpec {
        name: "get_user".to_string(),
        description: "Fetch user by id".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "id": { "type": "string" }
            },
            "required": ["id"]
        }),
        execution: ExecutionKind::GraphQL(mcp_factory_core::GraphQLOperation {
            document: "query($id: ID!) { user(id: $id) { name } }".to_string(),
            variable_bindings: vec!["id".to_string()],
        }),
    }
}

pub fn sample_resources() -> Vec<ResourceSpec> {
    vec![
        ResourceSpec {
            uri: "schema://openapi".to_string(),
            name: "openapi".to_string(),
            description: "OpenAPI schema".to_string(),
            mime_type: "application/yaml".to_string(),
            content: "openapi: 3.0.0",
        },
        ResourceSpec {
            uri: "meta://tools".to_string(),
            name: "tools".to_string(),
            description: "Tool index".to_string(),
            mime_type: "application/json".to_string(),
            content: r#"[{"name":"get_pet"}]"#,
        },
    ]
}

pub fn proxy_config(base_url: &str) -> ProxyConfig {
    ProxyConfig {
        base_url: base_url.to_string(),
        auth: AuthConfig::None,
        server_name: "test-server".to_string(),
        ..Default::default()
    }
}
