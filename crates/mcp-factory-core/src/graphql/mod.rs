use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::auth::AuthProvider;
use crate::config::ProxyConfig;
use crate::error::ProxyError;
use crate::tools::{validate_args, ExecutionKind, ToolSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLOperation {
    pub document: String,
    pub variable_bindings: Vec<String>,
}

pub struct GraphQLProxyExecutor {
    client: reqwest::Client,
    config: ProxyConfig,
    auth: Arc<dyn AuthProvider>,
}

impl GraphQLProxyExecutor {
    pub fn new(config: ProxyConfig, auth: Arc<dyn AuthProvider>) -> Result<Self, ProxyError> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout())
            .build()?;
        Ok(Self {
            client,
            config,
            auth,
        })
    }

    pub async fn execute(&self, tool: &ToolSpec, args: Value) -> Result<String, ProxyError> {
        let ExecutionKind::GraphQL(operation) = &tool.execution else {
            return Err(ProxyError::Other("expected GraphQL execution".to_string()));
        };
        validate_args(&tool.input_schema, &args)?;
        let variables = build_variables(operation, &args)?;
        let payload = json!({
            "query": operation.document,
            "variables": variables,
        });

        let mut request = self
            .client
            .post(&self.config.base_url)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .json(&payload);

        request = self.auth.apply_request_auth(request).await?;

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(ProxyError::Other(format!(
                "upstream returned {status}: {text}"
            )));
        }
        format_graphql_response(&text)
    }
}

pub fn build_variables(operation: &GraphQLOperation, args: &Value) -> Result<Value, ProxyError> {
    let Some(obj) = args.as_object() else {
        return Ok(Value::Object(Map::new()));
    };
    let mut variables = Map::new();
    for name in &operation.variable_bindings {
        if let Some(value) = obj.get(name) {
            variables.insert(name.clone(), value.clone());
        }
    }
    Ok(Value::Object(variables))
}

pub fn format_graphql_response(text: &str) -> Result<String, ProxyError> {
    let parsed: Value = serde_json::from_str(text)
        .map_err(|e| ProxyError::Other(format!("invalid GraphQL JSON response: {e}")))?;
    if let Some(errors) = parsed.get("errors") {
        if errors.as_array().is_some_and(|arr| !arr.is_empty()) {
            return Err(ProxyError::Other(format!("GraphQL errors: {errors}")));
        }
    }
    if let Some(data) = parsed.get("data") {
        return Ok(serde_json::to_string_pretty(data).unwrap_or_else(|_| data.to_string()));
    }
    Ok(text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn builds_variables_from_args() {
        let op = GraphQLOperation {
            document: "query($id: ID!) { user(id: $id) { name } }".to_string(),
            variable_bindings: vec!["id".to_string()],
        };
        let vars = build_variables(&op, &json!({"id": "1", "ignored": true})).unwrap();
        assert_eq!(vars, json!({"id": "1"}));
    }

    #[test]
    fn formats_successful_graphql_response() {
        let text = r#"{"data":{"user":{"name":"alice"}}}"#;
        let formatted = format_graphql_response(text).unwrap();
        assert!(formatted.contains("alice"));
    }

    #[test]
    fn surfaces_graphql_errors() {
        let text = r#"{"errors":[{"message":"not found"}]}"#;
        assert!(format_graphql_response(text).is_err());
    }
}
