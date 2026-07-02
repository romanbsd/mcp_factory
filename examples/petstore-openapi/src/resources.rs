use mcp_factory_core::ResourceSpec;

pub fn build_resources() -> Vec<ResourceSpec> {
    vec![
        ResourceSpec {
            uri: "schema://openapi".to_string(),
            name: "openapi".to_string(),
            description: "Embedded OpenAPI schema".to_string(),
            mime_type: "application/yaml".to_string(),
            content: "openapi: 3.0.3\ninfo:\n  title: Minimal API\n  version: 1.0.0\npaths:\n  /pets/{petId}:\n    get:\n      operationId: getPet\n      summary: Get a pet\n      parameters:\n        - name: petId\n          in: path\n          required: true\n          schema:\n            type: integer\n      responses:\n        \"200\":\n          description: OK\n",
        },
        ResourceSpec {
            uri: "meta://tools".to_string(),
            name: "tools".to_string(),
            description: "Generated tool index".to_string(),
            mime_type: "application/json".to_string(),
            content: "[\n  {\n    \"name\": \"getPet\",\n    \"description\": \"Get a pet\"\n  }\n]",
        },
    ]
}