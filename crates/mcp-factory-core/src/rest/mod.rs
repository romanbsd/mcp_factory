use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

use crate::auth::AuthProvider;
use crate::error::ProxyError;
use crate::tools::{ExecutionKind::Rest as ExecutionKindRest, ToolBody, ToolResult, ToolSpec};

/// Refuse to buffer an upstream response whose declared length exceeds this, so
/// a huge (or hostile) response can't exhaust memory. Chunked responses with no
/// Content-Length bypass the check — an acceptable best-effort guard.
pub(crate) const MAX_RESPONSE_BYTES: u64 = 64 * 1024 * 1024;

/// Path-segment encoding set: blocks `/` (and every other reserved/unsafe byte)
/// so a value can't inject a new path segment, but keeps the RFC 3986 unreserved
/// characters `- . _ ~` so dates/UUIDs/etc. aren't needlessly percent-mangled.
const PATH_VALUE: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~');

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
    base_url: String,
    auth: Arc<dyn AuthProvider>,
}

impl RestProxyExecutor {
    pub fn new(client: reqwest::Client, base_url: String, auth: Arc<dyn AuthProvider>) -> Self {
        Self {
            client,
            base_url,
            auth,
        }
    }

    pub async fn execute(&self, tool: &ToolSpec, args: Value) -> Result<ToolResult, ProxyError> {
        let ExecutionKindRest(operation) = &tool.execution else {
            return Err(ProxyError::Other("expected REST execution".to_string()));
        };
        let url = build_url(&self.base_url, operation, &args, self.auth.as_ref())?;
        let mut request = self
            .client
            .request(parse_method(&operation.method)?, &url)
            // Prefer JSON so we can attach structuredContent, but still accept
            // anything (e.g. binary downloads).
            .header(reqwest::header::ACCEPT, "application/json, */*");

        request = self.auth.apply_request_auth(request).await?;
        request = apply_headers(request, operation, &args)?;

        let content_type = operation
            .content_type
            .as_deref()
            .unwrap_or("application/json");
        if operation.raw_body {
            if let Some(body) = args.as_object().and_then(|obj| obj.get("body")) {
                request = apply_body(request, content_type, body);
            }
        } else if let Some(body) = build_body(operation, &args)? {
            request = apply_body(request, content_type, &body);
        }

        let response = request.send().await?;
        let status = response.status();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        // Surface navigation/quota headers the model can't otherwise see.
        let meta = collect_header_hints(response.headers());
        if let Some(len) = response.content_length() {
            if len > MAX_RESPONSE_BYTES {
                return Err(ProxyError::Other(format!(
                    "upstream response too large: {len} bytes"
                )));
            }
        }

        if !status.is_success() {
            let body = read_limited_text(response).await.unwrap_or_default();
            return Ok(error_result(status, &content_type, body, meta));
        }

        // 204 / empty success: report it explicitly rather than an empty string.
        if status == reqwest::StatusCode::NO_CONTENT {
            return Ok(ToolResult {
                meta,
                ..ToolResult::text(format!("{status}"))
            });
        }

        // Text passes through as a string (`.text()` honors the declared
        // charset); non-text is handed back as raw bytes so the server can wrap
        // it in a proper binary/image MCP content block instead of mangling it.
        let body = if is_texty(&content_type) {
            let text = read_limited_text(response).await?;
            // JSON objects also ride along as `structuredContent` so clients can
            // read fields directly. MCP requires structuredContent to be an
            // object, so arrays/scalars stay text-only.
            let structured = if content_type.to_ascii_lowercase().contains("json") {
                serde_json::from_str::<Value>(&text)
                    .ok()
                    .filter(Value::is_object)
            } else {
                None
            };
            ToolResult {
                structured,
                meta,
                ..ToolResult::text(text)
            }
        } else {
            ToolResult {
                body: ToolBody::Binary {
                    data: read_limited_bytes(response).await?,
                    mime: content_type,
                },
                structured: None,
                meta,
                is_error: false,
            }
        };
        Ok(body)
    }
}

pub(crate) async fn read_limited_text(response: reqwest::Response) -> Result<String, ProxyError> {
    read_limited_text_with_limit(response, MAX_RESPONSE_BYTES).await
}

pub(crate) async fn read_limited_text_with_limit(
    response: reqwest::Response,
    max_bytes: u64,
) -> Result<String, ProxyError> {
    let bytes = read_limited_response(response, max_bytes).await?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub(crate) async fn read_limited_bytes(response: reqwest::Response) -> Result<Vec<u8>, ProxyError> {
    read_limited_response(response, MAX_RESPONSE_BYTES).await
}

async fn read_limited_response(
    mut response: reqwest::Response,
    max_bytes: u64,
) -> Result<Vec<u8>, ProxyError> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        let new_len = body.len() + chunk.len();
        if new_len as u64 > max_bytes {
            return Err(ProxyError::Other(format!(
                "upstream response too large: more than {max_bytes} bytes"
            )));
        }
        body.extend_from_slice(&chunk);
    }
    Ok(body)
}

/// Whitelisted response headers that carry clues a client/LLM can act on:
/// where a created resource lives, pagination, rate-limit budget, caching.
const HINT_HEADERS: &[&str] = &[
    "location",
    "link",
    "retry-after",
    "etag",
    "x-ratelimit-remaining",
    "x-ratelimit-limit",
    "x-ratelimit-reset",
    "content-range",
];

fn collect_header_hints(headers: &reqwest::header::HeaderMap) -> Map<String, Value> {
    let mut meta = Map::new();
    for name in HINT_HEADERS {
        if let Some(value) = headers.get(*name).and_then(|v| v.to_str().ok()) {
            meta.insert(format!("http.{name}"), Value::String(value.to_string()));
        }
    }
    meta
}

/// Build a tool-level error result from a non-2xx response, with machine-usable
/// hints: whether it's worth retrying, any `Retry-After`, an auth cue, and the
/// parsed body when it's `application/problem+json` (RFC 7807).
fn error_result(
    status: reqwest::StatusCode,
    content_type: &str,
    body: String,
    meta: Map<String, Value>,
) -> ToolResult {
    let retryable = status.is_server_error() || status == reqwest::StatusCode::TOO_MANY_REQUESTS;
    let mut structured = Map::new();
    structured.insert("status".to_string(), Value::from(status.as_u16()));
    structured.insert("retryable".to_string(), Value::Bool(retryable));
    if matches!(
        status,
        reqwest::StatusCode::UNAUTHORIZED | reqwest::StatusCode::FORBIDDEN
    ) {
        structured.insert(
            "hint".to_string(),
            Value::String("authentication or authorization failed; re-authenticate".to_string()),
        );
    }
    if let Some(retry_after) = meta.get("http.retry-after") {
        structured.insert("retry_after".to_string(), retry_after.clone());
    }
    // Prefer a structured problem+json body; otherwise keep the raw text.
    if content_type.to_ascii_lowercase().contains("problem+json") {
        if let Ok(problem) = serde_json::from_str::<Value>(&body) {
            structured.insert("problem".to_string(), problem);
        }
    }
    ToolResult {
        body: ToolBody::Text(format!("upstream returned {status}: {body}")),
        structured: Some(Value::Object(structured)),
        meta,
        is_error: true,
    }
}

/// Attach a request body using the declared content type. Form bodies are
/// urlencoded; everything else is sent as JSON.
///
/// ponytail: only `application/x-www-form-urlencoded` is special-cased; nested
/// objects in a form body aren't flattened. Add multipart / deep-form support
/// if a real schema needs it.
fn apply_body(
    request: reqwest::RequestBuilder,
    content_type: &str,
    body: &Value,
) -> reqwest::RequestBuilder {
    if content_type == "application/x-www-form-urlencoded" {
        request.form(body)
    } else {
        request
            .header(reqwest::header::CONTENT_TYPE, content_type)
            .json(body)
    }
}

/// True for content types safe to hand back as a UTF-8 string. Anything else
/// (images, PDFs, octet-stream, ...) would be corrupted by lossy UTF-8 decoding,
/// so it is base64-encoded instead.
fn is_texty(content_type: &str) -> bool {
    let ct = content_type.to_ascii_lowercase();
    ct.is_empty()
        || ct.starts_with("text/")
        || ct.contains("json")
        || ct.contains("xml")
        || ct.contains("javascript")
        || ct.contains("csv")
        || ct.contains("urlencoded")
}

pub fn substitute_path(template: &str, args: &Value) -> Result<String, ProxyError> {
    let mut path = template.to_string();
    if let Some(obj) = args.as_object() {
        for (key, value) in obj {
            let placeholder = format!("{{{key}}}");
            if path.contains(&placeholder) {
                // Percent-encode per segment so a value like "../admin" or "a/b"
                // can't inject extra path segments (traversal / wrong endpoint).
                let replacement = encode_path_value(value)?;
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
                if binding.location != ParamLocation::Query {
                    continue;
                }
                let Some(value) = obj.get(&binding.name) else {
                    continue;
                };
                // Array query params expand to repeated pairs (?id=1&id=2), the
                // OpenAPI `form`/explode default; scalars append a single pair.
                match value {
                    Value::Array(items) => {
                        for item in items {
                            query_pairs.append_pair(&binding.name, &value_to_string(item)?);
                        }
                    }
                    _ => {
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

fn build_body(operation: &RestOperation, args: &Value) -> Result<Option<Value>, ProxyError> {
    if operation.body_fields.is_empty() {
        return Ok(None);
    }
    let Some(obj) = args.as_object() else {
        return Ok(None);
    };
    let mut body = Map::new();
    for field in &operation.body_fields {
        if let Some(value) = obj.get(field) {
            body.insert(field.clone(), value.clone());
        }
    }
    if body.is_empty() {
        Ok(None)
    } else {
        Ok(Some(Value::Object(body)))
    }
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

/// Encode a path-parameter value into a single URL segment. Arrays use the
/// OpenAPI `simple` style (comma-joined), each element individually encoded.
fn encode_path_value(value: &Value) -> Result<String, ProxyError> {
    match value {
        Value::Array(items) => {
            if items.is_empty() {
                return Err(ProxyError::Validation(
                    "empty array for path parameter".to_string(),
                ));
            }
            let encoded = items
                .iter()
                .map(
                    |item| Ok(utf8_percent_encode(&value_to_string(item)?, PATH_VALUE).to_string()),
                )
                .collect::<Result<Vec<_>, ProxyError>>()?;
            Ok(encoded.join(","))
        }
        _ => Ok(utf8_percent_encode(&value_to_string(value)?, PATH_VALUE).to_string()),
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
        // A slash in a path value must not create a new segment (no traversal);
        // unreserved chars like '.' and '-' are kept intact.
        let path = substitute_path("/pets/{id}", &json!({"id": "1/../admin"})).unwrap();
        assert_eq!(path, "/pets/1%2F..%2Fadmin");
        let dated = substitute_path("/logs/{day}", &json!({"day": "2020-01-01"})).unwrap();
        assert_eq!(dated, "/logs/2020-01-01");
    }

    #[test]
    fn array_path_param_uses_simple_style() {
        let path = substitute_path("/pets/{ids}", &json!({"ids": [1, 2, "a/b"]})).unwrap();
        assert_eq!(path, "/pets/1,2,a%2Fb");
    }

    #[test]
    fn array_query_param_expands_to_repeated_pairs() {
        let operation = RestOperation {
            method: "GET".to_string(),
            path_template: "/pets".to_string(),
            params: vec![ParamBinding {
                name: "tag".to_string(),
                location: ParamLocation::Query,
            }],
            body_fields: vec![],
            content_type: None,
            raw_body: false,
        };
        let auth =
            auth_provider_from_config(&AuthConfig::None, reqwest::Client::new(), false).unwrap();
        let url = build_url(
            "https://api.example.com/v1",
            &operation,
            &json!({"tag": ["dog", "cat"]}),
            auth.as_ref(),
        )
        .unwrap();
        assert_eq!(url, "https://api.example.com/v1/pets?tag=dog&tag=cat");
    }

    #[test]
    fn is_texty_classifies_content_types() {
        assert!(is_texty("application/json"));
        assert!(is_texty("text/html; charset=utf-8"));
        assert!(is_texty("")); // missing header defaults to text
        assert!(!is_texty("image/png"));
        assert!(!is_texty("application/octet-stream"));
    }

    #[tokio::test]
    async fn limited_stream_rejects_chunked_overflow() {
        let mock_server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string("1234567890x"))
            .mount(&mock_server)
            .await;
        let response = reqwest::Client::new()
            .get(mock_server.uri())
            .send()
            .await
            .unwrap();
        let err = read_limited_response(response, 10).await.unwrap_err();
        assert!(err.to_string().contains("upstream response too large"));
    }

    #[test]
    fn empty_array_path_param_errors() {
        assert!(substitute_path("/pets/{ids}", &json!({"ids": []})).is_err());
    }

    #[test]
    fn error_result_classifies_status() {
        let meta = Map::new();
        let server = error_result(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "text/plain",
            "boom".to_string(),
            meta.clone(),
        );
        assert!(server.is_error);
        let s = server.structured.unwrap();
        assert_eq!(s["status"], 500);
        assert_eq!(s["retryable"], true);

        let unauth = error_result(
            reqwest::StatusCode::UNAUTHORIZED,
            "text/plain",
            "nope".to_string(),
            meta,
        );
        let s = unauth.structured.unwrap();
        assert_eq!(s["retryable"], false);
        assert!(s["hint"].as_str().unwrap().contains("re-authenticate"));
    }

    #[test]
    fn error_result_parses_problem_json() {
        let s = error_result(
            reqwest::StatusCode::BAD_REQUEST,
            "application/problem+json",
            r#"{"title":"bad","detail":"x"}"#.to_string(),
            Map::new(),
        )
        .structured
        .unwrap();
        assert_eq!(s["problem"]["title"], "bad");
    }

    #[test]
    fn collect_header_hints_whitelists() {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("location", "/pets/1".parse().unwrap());
        headers.insert("x-custom", "ignored".parse().unwrap());
        let meta = collect_header_hints(&headers);
        assert_eq!(meta["http.location"], "/pets/1");
        assert!(!meta.contains_key("http.x-custom"));
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
        let auth =
            auth_provider_from_config(&AuthConfig::None, reqwest::Client::new(), false).unwrap();
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
        assert_eq!(body, Some(json!({"name": "fluffy", "tag": "dog"})));
    }

    #[test]
    fn skips_body_when_no_fields_are_present() {
        let operation = RestOperation {
            method: "POST".to_string(),
            path_template: "/pets".to_string(),
            params: vec![],
            body_fields: vec!["name".to_string()],
            content_type: Some("application/json".to_string()),
            raw_body: false,
        };
        assert_eq!(build_body(&operation, &json!({})).unwrap(), None);
    }
}
