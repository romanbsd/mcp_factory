use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::auth::AuthProvider;
use crate::error::ProxyError;
use crate::tools::{ExecutionKind, ToolResult, ToolSpec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphQLOperation {
    pub document: String,
    pub variable_bindings: Vec<String>,
}

pub struct GraphQLProxyExecutor {
    client: reqwest::Client,
    base_url: String,
    auth: Arc<dyn AuthProvider>,
}

impl GraphQLProxyExecutor {
    pub fn new(client: reqwest::Client, base_url: String, auth: Arc<dyn AuthProvider>) -> Self {
        Self {
            client,
            base_url,
            auth,
        }
    }

    pub async fn execute(&self, tool: &ToolSpec, args: Value) -> Result<ToolResult, ProxyError> {
        let ExecutionKind::GraphQL(operation) = &tool.execution else {
            return Err(ProxyError::Other("expected GraphQL execution".to_string()));
        };
        let variables = build_variables(operation, &args)?;
        let payload = json!({
            "query": operation.document,
            "variables": variables,
        });

        let mut request = self
            .client
            .post(&self.base_url)
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
        let formatted = format_graphql_response(&text)?;
        // Hand the `data` object back as structuredContent for direct field
        // access, keeping the pretty text for humans.
        let structured = serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|v| v.get("data").filter(|d| !d.is_null()).cloned());
        Ok(ToolResult::text(formatted).with_structured(structured))
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

    // GraphQL allows partial success: `data` and `errors` can both be present.
    let errors = parsed
        .get("errors")
        .filter(|e| e.as_array().is_some_and(|arr| !arr.is_empty()));
    let data = parsed.get("data").filter(|d| !d.is_null());

    let pretty = |v: &Value| serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string());
    match (data, errors) {
        // Partial success: return the data but keep the errors visible so the
        // caller knows the result is incomplete.
        (Some(data), Some(errors)) => Ok(pretty(&json!({"data": data, "errors": errors}))),
        (Some(data), None) => Ok(pretty(data)),
        (None, Some(errors)) => Err(ProxyError::Other(format!("GraphQL errors: {errors}"))),
        (None, None) => Ok(text.to_string()),
    }
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

    #[test]
    fn null_data_with_errors_is_error() {
        let text = r#"{"data":null,"errors":[{"message":"boom"}]}"#;
        assert!(format_graphql_response(text).is_err());
    }

    #[test]
    fn partial_data_with_errors_is_returned() {
        // data present alongside field-level errors must not be discarded, and
        // the errors stay visible so the caller knows it is partial.
        let text = r#"{"data":{"user":{"name":"alice"}},"errors":[{"message":"nickname failed"}]}"#;
        let formatted = format_graphql_response(text).unwrap();
        assert!(formatted.contains("alice"));
        assert!(formatted.contains("nickname failed"));
    }
}
