//! Interpret Google error evidence without treating every 403 as property denial.
use serde_json::{json, Value};
pub fn explain(response: &Value) -> Value {
    let error = response
        .get("body")
        .and_then(|v| v.get("error"))
        .or_else(|| response.get("error"))
        .unwrap_or(response);
    let status = response["status"]
        .as_u64()
        .or_else(|| error["code"].as_u64());
    let details = error["details"].as_array();
    let info = details.into_iter().flatten().find(|d| {
        d["@type"]
            .as_str()
            .is_some_and(|t| t.ends_with("google.rpc.ErrorInfo"))
    });
    let reason = info
        .and_then(|d| d["reason"].as_str())
        .or_else(|| error["errors"][0]["reason"].as_str())
        .unwrap_or("");
    let message = error["message"].as_str().unwrap_or("");
    let (category, remedy) = match reason {
        "SERVICE_DISABLED" | "accessNotConfigured" => ("api_disabled", "Enable the service named in Google's error in the consumer Cloud project, then retry."),
        "ACCESS_TOKEN_SCOPE_INSUFFICIENT" => ("insufficient_oauth_scope", "Re-authorize ADC with analytics.readonly (or the scope required by the raw operation), then restart the server."),
        "insufficientPermissions" if message.to_ascii_lowercase().contains("scope") => ("insufficient_oauth_scope", "Re-authorize ADC with the required Analytics OAuth scopes, then restart the server."),
        "RATE_LIMIT_EXCEEDED" | "RESOURCE_QUOTA_EXCEEDED" | "quotaExceeded" | "rateLimitExceeded" | "userRateLimitExceeded" | "dailyLimitExceeded" => ("quota_exhausted", "Inspect property quota and Retry-After; reduce reporting frequency or complexity."),
        "USER_PROJECT_DENIED" | "PROJECT_INVALID" | "CONSUMER_INVALID" | "BILLING_DISABLED" => ("credential_or_quota_project", "Check the ADC consumer/quota project, API enablement and serviceusage.services.use permission; configure auth.options.quota_project if required."),
        "ACCESS_TOKEN_EXPIRED" | "CREDENTIALS_MISSING" | "ACCESS_TOKEN_TYPE_UNSUPPORTED" | "API_KEY_INVALID" => ("invalid_credentials", "Refresh the credentials used by this MCP process and restart after ADC discovery failures."),
        _ if status == Some(401) => ("invalid_credentials", "Refresh the credentials used by this MCP process and restart after ADC discovery failures."),
        _ if status == Some(429) => ("quota_exhausted", "Inspect property quota and Retry-After; reduce reporting frequency or complexity."),
        _ if status == Some(403) && message.contains("permissions for this property") => ("property_permission_denied", "Grant the actual credential principal access to this GA4 property and verify the numeric property ID; Cloud IAM alone does not grant Analytics access."),
        _ if status == Some(403) => ("permission_denied_unclassified", "Google did not identify the cause precisely. Check API enablement, OAuth scopes, property access and ADC quota-project permissions; do not assume credentials are valid merely from HTTP 403."),
        _ => ("upstream_error", "Inspect the preserved upstream error; do not interpret unavailable reports as zero activity."),
    };
    json!({"category":category,"httpStatus":status,"googleReason":reason,"service":info.map(|v| &v["metadata"]["service"]),"consumer":info.map(|v| &v["metadata"]["consumer"]),"remedy":remedy})
}
