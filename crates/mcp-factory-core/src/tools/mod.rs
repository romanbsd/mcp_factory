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

/// Body of a tool result: UTF-8 text (JSON/XML/plain) or an opaque binary blob
/// with its MIME type. Keeping them distinct lets the server pick the right MCP
/// content block instead of stuffing raw bytes into a text field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolBody {
    Text(String),
    Binary { data: Vec<u8>, mime: String },
}

impl ToolBody {
    /// Flatten to a string; binary is base64-encoded.
    pub fn into_text(self) -> String {
        match self {
            ToolBody::Text(text) => text,
            ToolBody::Binary { data, .. } => {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(data)
            }
        }
    }
}

/// Everything an executor learned about a response, ready for the server to map
/// onto an MCP `CallToolResult`: the human-readable body, an optional parsed
/// `structured` value (for `structuredContent`), header `meta` hints, and
/// whether it is a tool-level error.
#[derive(Debug, Clone)]
pub struct ToolResult {
    pub body: ToolBody,
    pub structured: Option<Value>,
    pub meta: serde_json::Map<String, Value>,
    pub is_error: bool,
}

impl ToolResult {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            body: ToolBody::Text(text.into()),
            structured: None,
            meta: serde_json::Map::new(),
            is_error: false,
        }
    }

    pub fn with_structured(mut self, structured: Option<Value>) -> Self {
        self.structured = structured;
        self
    }

    /// Flatten to a string (binary → base64). Used where a plain string is
    /// expected, e.g. the `invoke_tool` test helper.
    pub fn into_text(self) -> String {
        self.body.into_text()
    }
}

/// Optional MCP metadata carried alongside a tool: a human title, a declared
/// output schema, and behavioral hints. All default to unset.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ToolHints {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destructive: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_world: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub execution: ExecutionKind,
    #[serde(default)]
    pub hints: ToolHints,
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

    pub fn register_many(
        &mut self,
        tools: impl IntoIterator<Item = ToolSpec>,
    ) -> Result<(), ProxyError> {
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
            hints: ToolHints::default(),
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
        let mut registry = ToolRegistry::new();
        registry.register(sample_tool("get_item")).unwrap();
        registry.validate("get_item", &json!({"id": "1"})).unwrap();
        assert!(registry.validate("get_item", &json!({})).is_err());
    }
}
