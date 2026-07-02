use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use crate::auth::AuthProvider;
use crate::config::ProxyConfig;
use crate::error::ProxyError;
use crate::tools::ToolSpec;

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
    /// When true, the single `body` argument is serialized verbatim as the
    /// request body (array/scalar/free-form bodies).
    #[serde(default)]
    pub raw_body: bool,
}

pub struct RestProxyExecutor {
    client: reqwest::Client,
    config: ProxyConfig,
    auth: Arc<dyn AuthProvider>,
}

impl RestProxyExecutor {
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
        let ExecutionKindRest(operation) = &tool.execution else {
            return Err(ProxyError::Other("expected REST execution".to_string()));
        };
        let url = build_url(
            &self.config.base_url,
            operation,
            &args,
            self.auth.as_ref(),
        )?;
        let mut request = self
            .client
            .request(parse_method(&operation.method)?, &url);

        request = self.auth.apply_request_auth(request).await?;
        request = apply_headers(request, operation, &args)?;

        if operation.raw_body {
            if let Some(body) = args.as_object().and_then(|obj| obj.get("body")) {
                let content_type = operation
                    .content_type
                    .as_deref()
                    .unwrap_or("application/json");
                request = request.header(reqwest::header::CONTENT_TYPE, content_type);
                request = request.json(body);
            }
        } else if !operation.body_fields.is_empty() {
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
                // Percent-encode per segment so a value like "../admin" or "a/b"
                // can't inject extra path segments (traversal / wrong endpoint).
                let raw = value_to_string(value)?;
                let replacement = utf8_percent_encode(&raw, NON_ALPHANUMERIC).to_string();
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
    auth: &dyn AuthProvider,
) -> Result<String, ProxyError> {
    let path = substitute_path(&operation.path_template, args)?;
    let mut url = reqwest::Url::parse(base_url)
        .map_err(|e| ProxyError::Config(format!("invalid base_url: {e}")))?;
    url.set_path(&join_paths(url.path(), &path));
    {
        let mut query_pairs = url.query_pairs_mut();
        if let Some((param, secret)) = auth.api_key_query() {
            query_pairs.append_pair(&param, &secret);
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
    use crate::auth::auth_provider_from_config;
    use crate::config::AuthConfig;
    use serde_json::json;

    #[test]
    fn substitutes_path_params() {
        let path = substitute_path("/pets/{id}", &json!({"id": "42"})).unwrap();
        assert_eq!(path, "/pets/42");
    }

    #[test]
    fn percent_encodes_path_params() {
        // A slash in a path value must not create a new segment (no traversal).
        let path = substitute_path("/pets/{id}", &json!({"id": "1/../admin"})).unwrap();
        assert_eq!(path, "/pets/1%2F%2E%2E%2Fadmin");
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
            raw_body: false,
        };
        let auth = auth_provider_from_config(&AuthConfig::None, reqwest::Client::new()).unwrap();
        let url = build_url(
            "https://api.example.com/v1",
            &operation,
            &json!({"limit": 10}),
            auth.as_ref(),
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
            raw_body: false,
        };
        let body = build_body(
            &operation,
            &json!({"name": "fluffy", "tag": "dog", "ignored": true}),
        )
        .unwrap();
        assert_eq!(body, json!({"name": "fluffy", "tag": "dog"}));
    }
}
