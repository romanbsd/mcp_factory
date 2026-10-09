//! Handwritten tools for App Store Connect protocols that are not plain
//! JSON:API CRUD: asset uploads and analytics segment downloads. This file is
//! preserved across `mcp-gen` runs; profiles for these tools are assigned in
//! tools.toml (`[custom_tools]`).

mod analytics;
mod asset_upload;

use std::time::Duration;

use mcp_factory_core::{CustomToolSpec, ProxyConfig, ProxyError, ToolResult};
use serde_json::{json, Value};

pub fn build_custom_tools(config: &ProxyConfig) -> Vec<CustomToolSpec> {
    // A client that cannot be built (e.g. TLS backend failure) is reported by
    // each call rather than aborting startup for the generated tools.
    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(config.timeout_secs.max(300)))
        // Every redirect hop must pass the same rule as the presigned URL.
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.previous().len() >= 5 {
                attempt.error("too many redirects")
            } else if check_transfer_url(attempt.url().as_str()).is_ok() {
                attempt.follow()
            } else {
                attempt.error("redirect target must use HTTPS")
            }
        }))
        .build()
        .map_err(|error| error.to_string());
    vec![
        asset_upload::tool(http.clone(), config),
        analytics::tool(http),
    ]
}

/// The shared transfer client, or a stage-tagged failure explaining why
/// it could not be created.
fn transfer_client(
    http: &Result<reqwest::Client, String>,
    stage: &str,
) -> Result<reqwest::Client, ToolResult> {
    http.clone()
        .map_err(|error| failure(stage, "HTTP client unavailable", json!(error)))
}

/// The parsed JSON of a generated tool's result, or a stage-tagged failure
/// carrying the upstream error (status, Apple `errors[]`) untouched.
fn json_of(result: ToolResult, stage: &str) -> Result<Value, ToolResult> {
    if result.is_error {
        return Err(failure(
            stage,
            "App Store Connect rejected the request",
            json!({"upstream": result.structured, "message": result.into_text()}),
        ));
    }
    match result.structured {
        Some(value) => Ok(value),
        None => serde_json::from_str(&result.into_text()).map_err(|_| {
            failure(
                stage,
                "App Store Connect returned a non-JSON response",
                Value::Null,
            )
        }),
    }
}

/// A tool-level error that names the failed stage.
fn failure(stage: &str, message: &str, details: Value) -> ToolResult {
    let body = json!({"ok": false, "stage": stage, "message": message, "details": details});
    ToolResult {
        is_error: true,
        ..ToolResult::text(body.to_string()).with_structured(Some(body))
    }
}

fn success(body: Value) -> ToolResult {
    ToolResult::text(body.to_string()).with_structured(Some(body))
}

/// Presigned upload/download URLs come from Apple's responses; refuse anything
/// but HTTPS (loopback HTTP is allowed so tests can use a mock server).
fn check_transfer_url(url: &str) -> Result<reqwest::Url, String> {
    let parsed = reqwest::Url::parse(url).map_err(|_| "invalid transfer URL".to_string())?;
    let loopback = matches!(parsed.host_str(), Some("127.0.0.1" | "localhost" | "[::1]"));
    match parsed.scheme() {
        "https" => Ok(parsed),
        "http" if loopback => Ok(parsed),
        _ => Err("transfer URL must use HTTPS".to_string()),
    }
}

fn proxy_error_result(stage: &str, error: ProxyError) -> ToolResult {
    failure(stage, &error.to_string(), Value::Null)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn transfer_urls_must_be_https_except_loopback() {
        assert!(check_transfer_url("https://upload.example.com/x").is_ok());
        assert!(check_transfer_url("http://127.0.0.1:9000/x").is_ok());
        assert!(check_transfer_url("http://upload.example.com/x").is_err());
        assert!(check_transfer_url("file:///etc/passwd").is_err());
    }
}
