use mcp_factory_core::ResourceSpec;

pub fn build_resources() -> Vec<ResourceSpec> {
    vec![
        ResourceSpec {
            uri: "schema://graphql".to_string(),
            name: "graphql".to_string(),
            description: "Embedded GraphQL schema".to_string(),
            mime_type: "text/plain".to_string(),
            content: "type Query {\n  user(id: ID!): User\n}\n\ntype Mutation {\n  createUser(name: String!): User\n}\n\ntype User {\n  id: ID!\n  name: String!\n}\n",
        },
        ResourceSpec {
            uri: "meta://tools".to_string(),
            name: "tools".to_string(),
            description: "Generated tool index".to_string(),
            mime_type: "application/json".to_string(),
            content: "[\n  {\n    \"name\": \"user\",\n    \"description\": \"GraphQL query user\"\n  },\n  {\n    \"name\": \"createUser\",\n    \"description\": \"GraphQL mutation createUser\"\n  }\n]",
        },
    ]
}