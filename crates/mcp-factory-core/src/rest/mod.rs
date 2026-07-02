use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::config::{AuthConfig, ProxyConfig};
use crate::error::ProxyError;
use crate::tools::{validate_args, ToolSpec};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamLocation {
    Path,
    Query,
    Header,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamBinding {
    pub name: String,
    pub location: ParamLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestOperation {
    pub method: String,
    pub path_template: String,
    pub params: Vec<ParamBinding>,
    pub body_fields: Vec<String>,
    #[serde(default)]
    pub content_type: Option<String>,
}

pub struct RestProxyExecutor {
    client: reqwest::Client,
    config: ProxyConfig,
}

impl RestProxyExecutor {
    pub fn new(config: ProxyConfig) -> Result<Self, ProxyError> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout())
            .build()?;
        Ok(Self { client, config })
    }

    pub async fn execute(&self, tool: &ToolSpec, args: Value) -> Result<String, ProxyError> {
        let ExecutionKindRest(operation) = &tool.execution else {
            return Err(ProxyError::Other("expected REST execution".to_string()));
        };
        validate_args(&tool.input_schema, &args)?;
        let url = build_url(&self.config.base_url, operation, &args, &self.config.auth)?;
        let mut request = self
            .client
            .request(parse_method(&operation.method)?, &url);

        request = apply_auth(request, &self.config.auth)?;
        request = apply_headers(request, operation, &args)?;

        if !operation.body_fields.is_empty() {
            let body = build_body(operation, &args)?;
            let content_type = operation
                .content_type
                .as_deref()
                .unwrap_or("application/json");
            request = request.header(reqwest::header::CONTENT_TYPE, content_type);
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if status.is_success() {
            Ok(text)
        } else {
            Err(ProxyError::Other(format!(
                "upstream returned {status}: {text}"
            )))
        }
    }
}

use crate::tools::ExecutionKind::Rest as ExecutionKindRest;

pub fn substitute_path(template: &str, args: &Value) -> Result<String, ProxyError> {
    let mut path = template.to_string();
    if let Some(obj) = args.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{key}}}");
            if path.contains(&placeholder) {
                let replacement = value_to_string(value)?;
                path = path.replace(&placeholder, &replacement);
            }
        }
    }
    if path.contains('{') {
        return Err(ProxyError::Validation(
            "missing required path parameters".to_string(),
        ));
    }
    Ok(path)
}

pub fn build_url(
    base_url: &str,
    operation: &RestOperation,
    args: &Value,
    auth: &AuthConfig,
) -> Result<String, ProxyError> {
    let path = substitute_path(&operation.path_template, args)?;
    let mut url = reqwest::Url::parse(base_url)
        .map_err(|e| ProxyError::Config(format!("invalid base_url: {e}")))?;
    url.set_path(&join_paths(url.path(), &path));
    {
        let mut query_pairs = url.query_pairs_mut();
        if let AuthConfig::ApiKeyQuery { param, .. } = auth {
            if let Some(secret) = auth.resolve_secret() {
                query_pairs.append_pair(param, &secret);
            }
        }
        if let Some(obj) = args.as_object() {
            for binding in &operation.params {
                if binding.location == ParamLocation::Query {
                    if let Some(value) = obj.get(&binding.name) {
                        query_pairs.append_pair(&binding.name, &value_to_string(value)?);
                    }
                }
            }
        }
    }
    Ok(url.to_string())
}

fn join_paths(base: &str, path: &str) -> String {
    let base = base.trim_end_matches('/');
    let path = path.trim_start_matches('/');
    if base.is_empty() {
        format!("/{path}")
    } else {
        format!("{base}/{path}")
    }
}

fn apply_auth(
    mut request: reqwest::RequestBuilder,
    auth: &AuthConfig,
) -> Result<reqwest::RequestBuilder, ProxyError> {
    match auth {
        AuthConfig::None => {}
        AuthConfig::Bearer { .. } => {
            if let Some(token) = auth.resolve_secret() {
                request = request.bearer_auth(token);
            }
        }
        AuthConfig::ApiKeyHeader { header, .. } => {
            if let Some(key) = auth.resolve_secret() {
                request = request.header(header, key);
            }
        }
        AuthConfig::ApiKeyQuery { .. } => {}
    }
    Ok(request)
}

fn apply_headers(
    mut request: reqwest::RequestBuilder,
    operation: &RestOperation,
    args: &Value,
) -> Result<reqwest::RequestBuilder, ProxyError> {
    let Some(obj) = args.as_object() else {
        return Ok(request);
    };
    for binding in &operation.params {
        if binding.location == ParamLocation::Header {
            if let Some(value) = obj.get(&binding.name) {
                request = request.header(&binding.name, value_to_string(value)?);
            }
        }
    }
    Ok(request)
}

fn build_body(operation: &RestOperation, args: &Value) -> Result<Value, ProxyError> {
    let Some(obj) = args.as_object() else {
        return Ok(Value::Object(Map::new()));
    };
    let mut body = Map::new();
    for field in &operation.body_fields {
        if let Some(value) = obj.get(field) {
            body.insert(field.clone(), value.clone());
        }
    }
    Ok(Value::Object(body))
}

fn parse_method(method: &str) -> Result<reqwest::Method, ProxyError> {
    reqwest::Method::from_bytes(method.as_bytes())
        .map_err(|_| ProxyError::Config(format!("invalid HTTP method: {method}")))
}

fn value_to_string(value: &Value) -> Result<String, ProxyError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        other => Err(ProxyError::Validation(format!(
            "cannot convert value to string: {other}"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn substitutes_path_params() {
        let path = substitute_path("/pets/{id}", &json!({"id": "42"})).unwrap();
        assert_eq!(path, "/pets/42");
    }

    #[test]
    fn builds_query_string() {
        let operation = RestOperation {
            method: "GET".to_string(),
            path_template: "/pets".to_string(),
            params: vec![ParamBinding {
                name: "limit".to_string(),
                location: ParamLocation::Query,
            }],
            body_fields: vec![],
            content_type: None,
        };
        let url = build_url(
            "https://api.example.com/v1",
            &operation,
            &json!({"limit": 10}),
            &AuthConfig::None,
        )
        .unwrap();
        assert_eq!(url, "https://api.example.com/v1/pets?limit=10");
    }

    #[test]
    fn builds_json_body_from_fields() {
        let operation = RestOperation {
            method: "POST".to_string(),
            path_template: "/pets".to_string(),
            params: vec![],
            body_fields: vec!["name".to_string(), "tag".to_string()],
            content_type: Some("application/json".to_string()),
        };
        let body = build_body(
            &operation,
            &json!({"name": "fluffy", "tag": "dog", "ignored": true}),
        )
        .unwrap();
        assert_eq!(body, json!({"name": "fluffy", "tag": "dog"}));
    }
}
