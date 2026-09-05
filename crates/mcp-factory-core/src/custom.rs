use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use serde_json::Value;

use crate::error::ProxyError;
use crate::tools::{ToolHints, ToolResult};

/// Restricted invocation surface exposed to handwritten orchestration tools.
/// Implementations must reject generated tools that are not explicitly marked
/// read-only, so reporting code cannot mutate an upstream by construction.
#[async_trait]
pub trait ReadOnlyToolInvoker: Send + Sync {
    async fn invoke_read_only(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError>;
}

/// Async implementation for a handwritten MCP tool.
#[async_trait]
pub trait CustomToolHandler: Send + Sync {
    async fn call(
        &self,
        invoker: &dyn ReadOnlyToolInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError>;
}

/// Metadata and implementation for one handwritten MCP tool.
#[derive(Clone)]
pub struct CustomToolSpec {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
    pub hints: ToolHints,
    pub handler: Arc<dyn CustomToolHandler>,
}

#[derive(Default)]
pub(crate) struct CustomToolRegistry {
    tools: HashMap<String, CustomToolSpec>,
    validators: HashMap<String, jsonschema::Validator>,
}

impl CustomToolRegistry {
    pub(crate) fn register(&mut self, tool: CustomToolSpec) -> Result<(), ProxyError> {
        if self.tools.contains_key(&tool.name) {
            return Err(ProxyError::DuplicateTool(tool.name.clone()));
        }
        let validator = jsonschema::validator_for(&tool.input_schema).map_err(|error| {
            ProxyError::Validation(format!("invalid schema for tool {}: {error}", tool.name))
        })?;
        // Validate a declared output schema the same way as the input schema so a
        // non-object or malformed output_schema fails registration, not later
        // during list_tools discovery.
        if let Some(output_schema) = &tool.hints.output_schema {
            if !output_schema.is_object() {
                return Err(ProxyError::Validation(format!(
                    "invalid output schema for tool {}: output_schema must be an object",
                    tool.name
                )));
            }
            jsonschema::validator_for(output_schema).map_err(|error| {
                ProxyError::Validation(format!(
                    "invalid output schema for tool {}: {error}",
                    tool.name
                ))
            })?;
        }
        self.validators.insert(tool.name.clone(), validator);
        self.tools.insert(tool.name.clone(), tool);
        Ok(())
    }

    pub(crate) fn register_many(
        &mut self,
        tools: impl IntoIterator<Item = CustomToolSpec>,
    ) -> Result<(), ProxyError> {
        for tool in tools {
            self.register(tool)?;
        }
        Ok(())
    }

    pub(crate) fn validate(&self, name: &str, arguments: &Value) -> Result<(), ProxyError> {
        if let Some(validator) = self.validators.get(name) {
            validator.validate(arguments).map_err(|error| {
                ProxyError::Validation(format!("invalid arguments for {name}: {error}"))
            })?;
        }
        Ok(())
    }

    pub(crate) fn get(&self, name: &str) -> Option<&CustomToolSpec> {
        self.tools.get(name)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = &CustomToolSpec> {
        self.tools.values()
    }

    pub(crate) fn len(&self) -> usize {
        self.tools.len()
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct NoopHandler;

    #[async_trait]
    impl CustomToolHandler for NoopHandler {
        async fn call(
            &self,
            _invoker: &dyn ReadOnlyToolInvoker,
            _arguments: Value,
        ) -> Result<ToolResult, ProxyError> {
            Ok(ToolResult::text("ok"))
        }
    }

    fn spec(hints: ToolHints) -> CustomToolSpec {
        CustomToolSpec {
            name: "custom".to_string(),
            description: "c".to_string(),
            input_schema: json!({"type": "object"}),
            hints,
            handler: Arc::new(NoopHandler),
        }
    }

    #[test]
    fn non_object_output_schema_fails_registration() {
        let mut registry = CustomToolRegistry::default();
        let err = registry
            .register(spec(ToolHints {
                output_schema: Some(json!("not-an-object")),
                ..Default::default()
            }))
            .unwrap_err();
        assert!(matches!(err, ProxyError::Validation(_)));
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn valid_output_schema_registers() {
        let mut registry = CustomToolRegistry::default();
        registry
            .register(spec(ToolHints {
                output_schema: Some(json!({"type": "object"})),
                ..Default::default()
            }))
            .unwrap();
        assert_eq!(registry.len(), 1);
    }
}
