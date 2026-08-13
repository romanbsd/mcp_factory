use mcp_factory_core::{
    ExecutionKind, ParamBinding, ParamLocation, RestOperation, ToolHints, ToolSpec,
};
pub fn build_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "getPet".to_string(),
            description: "Get a pet".to_string(),
            input_schema: serde_json::from_str(r#"{"properties":{"petId":{"type":"integer"}},"required":["petId"],"type":"object"}"#)
                .expect("generated input schema must be valid JSON"),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/pets/{petId}".to_string(),
                params: vec![
                    ParamBinding {
                        name: "petId".to_string(),
                        location: ParamLocation::Path,
                    },
                ],
                body_fields: vec![
                ],
                content_type: None,
                raw_body: false,
            }),
            hints: ToolHints {
                title: Some("Get a pet".to_string()),
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        },
    ]
}