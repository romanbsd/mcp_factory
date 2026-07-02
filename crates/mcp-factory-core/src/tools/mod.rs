use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ProxyError;
use crate::graphql::GraphQLOperation;
use crate::rest::RestOperation;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExecutionKind {
    Rest(RestOperation),
    GraphQL(GraphQLOperation),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub execution: ExecutionKind,
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: HashMap<String, ToolSpec>,
    // Compiled once at registration instead of per tool call.
    validators: HashMap<String, jsonschema::Validator>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, tool: ToolSpec) -> Result<(), ProxyError> {
        if self.tools.contains_key(&tool.name) {
            return Err(ProxyError::DuplicateTool(tool.name.clone()));
        }
        let validator = jsonschema::validator_for(&tool.input_schema).map_err(|e| {
            ProxyError::Validation(format!("invalid schema for tool {}: {e}", tool.name))
        })?;
        self.validators.insert(tool.name.clone(), validator);
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    /// Validate args against a tool's precompiled input schema.
    pub fn validate(&self, name: &str, args: &Value) -> Result<(), ProxyError> {
        if let Some(validator) = self.validators.get(name) {
            if let Err(error) = validator.validate(args) {
                return Err(ProxyError::Validation(error.to_string()));
            }
        }
        Ok(())
    }

    pub fn register_many(&mut self, tools: impl IntoIterator<Item = ToolSpec>) -> Result<(), ProxyError> {
        for tool in tools {
            self.register(tool)?;
        }
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&ToolSpec> {
        self.tools.get(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ToolSpec> {
        self.tools.values()
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

pub fn validate_args(schema: &Value, args: &Value) -> Result<(), ProxyError> {
    let validator = jsonschema::validator_for(schema)
        .map_err(|e| ProxyError::Validation(format!("invalid schema: {e}")))?;
    if let Err(error) = validator.validate(args) {
        return Err(ProxyError::Validation(error.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::{ParamBinding, ParamLocation, RestOperation};
    use serde_json::json;

    fn sample_tool(name: &str) -> ToolSpec {
        ToolSpec {
            name: name.to_string(),
            description: format!("tool {name}"),
            input_schema: json!({"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]}),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/items/{id}".to_string(),
                params: vec![ParamBinding {
                    name: "id".to_string(),
                    location: ParamLocation::Path,
                }],
                body_fields: vec![],
                content_type: None,
                raw_body: false,
            }),
        }
    }

    #[test]
    fn rejects_duplicate_tool_names() {
        let mut registry = ToolRegistry::new();
        registry.register(sample_tool("get_item")).unwrap();
        assert!(registry.register(sample_tool("get_item")).is_err());
    }

    #[test]
    fn validates_args_against_schema() {
        let schema = json!({"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]});
        validate_args(&schema, &json!({"id": "1"})).unwrap();
        assert!(validate_args(&schema, &json!({})).is_err());
    }
}
