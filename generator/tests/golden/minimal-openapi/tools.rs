use mcp_factory_core::ExecutionKind;
use mcp_factory_core::ParamBinding;
use mcp_factory_core::ParamLocation;
use mcp_factory_core::RestOperation;
use mcp_factory_core::ToolHints;
use mcp_factory_core::ToolSpec;
#[allow(dead_code)]
pub fn build_tools() -> Vec<ToolSpec> {
    operation_catalog().iter().map(|operation| (operation.build)()).collect()
}

/// All generated capabilities; creating this catalog does not parse any schema.
pub fn operation_catalog() -> &'static [mcp_factory_core::OperationDescriptor] {
    &[
        mcp_factory_core::OperationDescriptor { name: "getPet", build: build_operation_0 },
    ]
}
fn build_operation_0() -> ToolSpec {
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
            media: None,
        }),
            hints: ToolHints {
                title: Some("Get a pet".to_string()),
                output_schema: None,
                read_only: Some(true),
                destructive: Some(false),
                idempotent: Some(true),
                open_world: Some(true),
            },
        }
}
