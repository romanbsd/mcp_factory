use mcp_factory_core::{
    ExecutionKind, GraphQLOperation, ToolSpec,
};
use serde_json::json;

pub fn build_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec {
            name: "user".to_string(),
            description: "GraphQL query user".to_string(),
            input_schema: json!({"properties": {"id": {"type": "string"}}, "required": ["id"], "type": "object"}),
            execution: ExecutionKind::GraphQL(GraphQLOperation {
                document: "query($id: ID) { user(id: $id) { id name } }".to_string(),
                variable_bindings: vec![
                    "id".to_string(),
                ],
            }),
        },
        ToolSpec {
            name: "createUser".to_string(),
            description: "GraphQL mutation createUser".to_string(),
            input_schema: json!({"properties": {"name": {"type": "string"}}, "required": ["name"], "type": "object"}),
            execution: ExecutionKind::GraphQL(GraphQLOperation {
                document: "mutation($name: String) { createUser(name: $name) { id name } }".to_string(),
                variable_bindings: vec![
                    "name".to_string(),
                ],
            }),
        },
    ]
}