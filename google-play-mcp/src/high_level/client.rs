use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use mcp_factory_core::chrono::Utc;
use mcp_factory_core::{ProxyError, ReadOnlyToolInvoker, ToolBody, ToolResult};
use serde_json::{json, Map, Value};

use super::registry;

#[derive(Default)]
struct ClientState {
    source_calls: Vec<Value>,
    warnings: Vec<Value>,
    successes: usize,
    failures: usize,
}

/// `call`/`call_pages` take `&self` (bookkeeping lives behind a `Mutex`, never
/// held across an `.await`) so callers can run several source calls
/// concurrently via `join!`/`join_all` instead of one at a time — the read-only
/// Google APIs behind a report are independent, and the wall-clock cost of a
/// report is dominated by serial network latency, not any shared state.
pub struct EvidenceClient<'a> {
    invoker: &'a dyn ReadOnlyToolInvoker,
    state: Mutex<ClientState>,
}

impl<'a> EvidenceClient<'a> {
    pub fn new(invoker: &'a dyn ReadOnlyToolInvoker) -> Self {
        Self {
            invoker,
            state: Mutex::new(ClientState::default()),
        }
    }

    pub fn status(&self) -> &'static str {
        let state = self.state.lock().unwrap();
        match (state.successes, state.failures) {
            (0, 1..) => "unavailable",
            (_, 1..) => "partial",
            _ => "complete",
        }
    }

    pub fn source_calls(&self) -> Vec<Value> {
        self.state.lock().unwrap().source_calls.clone()
    }

    pub fn warnings(&self) -> Vec<Value> {
        self.state.lock().unwrap().warnings.clone()
    }

    pub async fn call(&self, id: &str, method: &str, arguments: Value) -> Option<Value> {
        let started = Instant::now();
        let started_at = Utc::now().to_rfc3339();
        if !registry::allowed(method) {
            let mut state = self.state.lock().unwrap();
            state.failures += 1;
            state.source_calls.push(call_record(
                id,
                method,
                &arguments,
                &started_at,
                started.elapsed(),
                CallOutcome {
                    attempts: 0,
                    result_state: "blocked",
                    pagination_complete: true,
                    error: Some(json!({
                        "message": "Method is outside the high-level read-only allowlist"
                    })),
                },
            ));
            return None;
        }
        let mut attempts = 0_u64;
        loop {
            attempts += 1;
            match self
                .invoker
                .invoke_read_only(method, arguments.clone())
                .await
            {
                Ok(result) if !result.is_error => {
                    match result_value(result) {
                        Ok(value) => {
                            let mut state = self.state.lock().unwrap();
                            state.successes += 1;
                            state.source_calls.push(call_record(
                                id,
                                method,
                                &arguments,
                                &started_at,
                                started.elapsed(),
                                CallOutcome {
                                    attempts,
                                    result_state: "success",
                                    pagination_complete: true,
                                    error: None,
                                },
                            ));
                            return Some(value);
                        }
                        Err(error) => {
                            let mut state = self.state.lock().unwrap();
                            state.failures += 1;
                            let error = json!({"message": error.to_string()});
                            state.source_calls.push(call_record(
                                id,
                                method,
                                &arguments,
                                &started_at,
                                started.elapsed(),
                                CallOutcome {
                                    attempts,
                                    result_state: "error",
                                    pagination_complete: true,
                                    error: Some(error),
                                },
                            ));
                            state.warnings.push(json!({
                                "sourceCall": id,
                                "message": "Source returned a non-JSON response"
                            }));
                            return None;
                        }
                    }
                }
                Ok(result) => {
                    let error = error_value(&result);
                    if attempts < 3 && transient_error(&error) {
                        retry_delay(attempts, &error).await;
                        continue;
                    }
                    let mut state = self.state.lock().unwrap();
                    state.failures += 1;
                    state.source_calls.push(call_record(
                        id,
                        method,
                        &arguments,
                        &started_at,
                        started.elapsed(),
                        CallOutcome {
                            attempts,
                            result_state: "error",
                            pagination_complete: true,
                            error: Some(error),
                        },
                    ));
                    return None;
                }
                Err(error) => {
                    let value = json!({"message": error.to_string()});
                    if attempts < 3 && transient_error(&value) {
                        retry_delay(attempts, &value).await;
                        continue;
                    }
                    let mut state = self.state.lock().unwrap();
                    state.failures += 1;
                    state.source_calls.push(call_record(
                        id,
                        method,
                        &arguments,
                        &started_at,
                        started.elapsed(),
                        CallOutcome {
                            attempts,
                            result_state: "error",
                            pagination_complete: true,
                            error: Some(value),
                        },
                    ));
                    return None;
                }
            }
        }
    }

    /// Pages through `method`, inserting the next-page token at
    /// `request_token_path` in the arguments and reading it back from
    /// `response_token_path` in each page. The request path is a slice so the
    /// token can be nested — e.g. `&["body", "pageToken"]` for the vitals
    /// `*_query` tools whose HTTP body is `arguments["body"]` (raw_body), versus
    /// `&["pageToken"]` for GET list/search tools that take it as a query param.
    pub async fn call_pages(
        &self,
        id: &str,
        method: &str,
        mut arguments: Value,
        request_token_path: &[&str],
        response_token_path: &[&str],
    ) -> (Vec<Value>, bool) {
        let mut pages = Vec::new();
        for page_number in 1..=100 {
            let call_id = format!("{id}-page-{page_number}");
            let Some(page) = self.call(&call_id, method, arguments.clone()).await else {
                let mut state = self.state.lock().unwrap();
                if let Some(call) = state.source_calls.last_mut() {
                    call["paginationComplete"] = Value::Bool(false);
                }
                return (pages, false);
            };
            let next = value_at_path(&page, response_token_path)
                .and_then(Value::as_str)
                .filter(|token| !token.is_empty())
                .map(str::to_string);
            pages.push(page);
            let Some(next) = next else {
                return (pages, true);
            };
            // A token that cannot be placed (misconfigured path, non-object
            // intermediate) would otherwise loop on the same page until the cap.
            if !set_at_path(&mut arguments, request_token_path, Value::String(next)) {
                self.state.lock().unwrap().warnings.push(json!({
                    "sourceCall": id,
                    "message": format!(
                        "Could not place next-page token at {:?}; pagination stopped early",
                        request_token_path
                    )
                }));
                return (pages, false);
            }
        }
        {
            let mut state = self.state.lock().unwrap();
            if let Some(call) = state.source_calls.last_mut() {
                call["paginationComplete"] = Value::Bool(false);
            }
            state.warnings.push(json!({
                "sourceCall": id,
                "message": "Pagination stopped after 100 pages"
            }));
        }
        (pages, false)
    }
}

fn result_value(result: ToolResult) -> Result<Value, ProxyError> {
    if let Some(value) = result.structured {
        return Ok(value);
    }
    match result.body {
        ToolBody::Text(text) => serde_json::from_str(&text)
            .map_err(|error| ProxyError::Other(format!("source returned invalid JSON: {error}"))),
        ToolBody::Binary { .. } => Err(ProxyError::Other(
            "report source returned unexpected binary data".to_string(),
        )),
    }
}

fn error_value(result: &ToolResult) -> Value {
    let message = match &result.body {
        ToolBody::Text(text) => text.clone(),
        ToolBody::Binary { .. } => "binary error response".to_string(),
    };
    let mut error = result
        .structured
        .clone()
        .and_then(|value| value.as_object().cloned())
        .unwrap_or_default();
    error.insert("message".to_string(), Value::String(message));
    Value::Object(error)
}

fn transient_error(error: &Value) -> bool {
    let status = error.get("status").and_then(Value::as_u64);
    if matches!(status, Some(429 | 500 | 502 | 503 | 504)) {
        return true;
    }
    let text = error.to_string();
    text.contains("UNAVAILABLE")
        || text.contains("RESOURCE_EXHAUSTED")
        || (text.contains("SERVICE_DISABLED")
            && (text.contains("activationUrl") || text.contains("serviceusage")))
        // Some androidpublisher endpoints (e.g. applications.tracks.releases.list)
        // report quota exhaustion as 403 PERMISSION_DENIED instead of 429
        // RESOURCE_EXHAUSTED — Google's own guidance treats these the same:
        // back off and retry rather than fail immediately.
        || (status == Some(403) && text.to_ascii_lowercase().contains("quota"))
}

async fn retry_delay(attempt: u64, error: &Value) {
    let guided_millis = error
        .get("retry_after")
        .and_then(Value::as_str)
        .and_then(|value| value.parse::<f64>().ok())
        .map(|seconds| (seconds * 1_000.0) as u64)
        .map(|millis| millis.min(5_000));
    let base_millis = guided_millis.unwrap_or(if attempt == 1 { 100 } else { 400 });
    let jitter = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| u64::from(duration.subsec_nanos()) % 50);
    let millis = base_millis.saturating_add(jitter);
    tokio::time::sleep(Duration::from_millis(millis)).await;
}

struct CallOutcome {
    attempts: u64,
    result_state: &'static str,
    pagination_complete: bool,
    error: Option<Value>,
}

fn call_record(
    id: &str,
    method: &str,
    arguments: &Value,
    started_at: &str,
    elapsed: Duration,
    outcome: CallOutcome,
) -> Value {
    json!({
        "id": id,
        "method": method,
        "arguments": redact(arguments),
        "startedAt": started_at,
        "finishedAt": Utc::now().to_rfc3339(),
        "attempts": outcome.attempts,
        "resultState": outcome.result_state,
        "elapsedMs": elapsed.as_millis() as u64,
        "freshness": null,
        "paginationComplete": outcome.pagination_complete,
        "error": outcome.error.as_ref().map(redact),
    })
}

fn redact(value: &Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| {
                    let normalized = key.to_ascii_lowercase();
                    let hidden = normalized.contains("token")
                        || normalized.contains("secret")
                        || normalized.contains("credential")
                        || normalized.contains("orderid")
                        || normalized.contains("reviewtext");
                    (
                        key.clone(),
                        if hidden {
                            Value::String("[redacted]".to_string())
                        } else {
                            redact(value)
                        },
                    )
                })
                .collect::<Map<_, _>>(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(redact).collect()),
        Value::String(text)
            if text.contains("Bearer ")
                || text.contains("ya29.")
                || text.contains("-----BEGIN PRIVATE KEY-----") =>
        {
            Value::String("[redacted]".to_string())
        }
        other => other.clone(),
    }
}

fn value_at_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter().try_fold(value, |current, key| current.get(key))
}

/// Sets `value` at `path`, creating intermediate objects as needed. Returns
/// false when the path is empty or an intermediate node is not an object.
fn set_at_path(root: &mut Value, path: &[&str], value: Value) -> bool {
    let Some((last, parents)) = path.split_last() else {
        return false;
    };
    let mut current = root;
    for key in parents {
        if !current.is_object() {
            return false;
        }
        current = current
            .as_object_mut()
            .unwrap()
            .entry((*key).to_string())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    if let Some(object) = current.as_object_mut() {
        object.insert((*last).to_string(), value);
        return true;
    }
    false
}
