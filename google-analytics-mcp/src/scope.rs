//! Explicit event-data scope; never infer Android from a mixed property.
use mcp_factory_core::ProxyError;
use serde_json::{json, Value};

pub fn supported(tool: &str) -> bool {
    matches!(
        tool,
        "analytics_overview"
            | "analytics_acquisition"
            | "analytics_content"
            | "analytics_events"
            | "analytics_ecommerce"
            | "analytics_run_report"
            | "analytics_compare"
            | "analytics_batch"
            | "analytics_funnel"
            | "analytics_pivot"
            | "analytics_conversions"
            | "analytics_activation"
            | "analytics_retention"
    )
}
pub fn apply(body: &mut Value, args: &Value) -> Result<(), ProxyError> {
    let mut filters = Vec::new();
    if let Some(platform) = args.get("platform") {
        let platform = platform
            .as_str()
            .filter(|p| ["Android", "iOS", "web"].contains(p))
            .ok_or_else(|| ProxyError::Validation("platform must be Android, iOS or web".into()))?;
        filters.push(exact("platform", platform));
    }
    if let Some(stream) = args.get("streamId") {
        let stream = stream
            .as_str()
            .filter(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
            .ok_or_else(|| ProxyError::Validation("streamId must be a numeric string".into()))?;
        filters.push(exact("streamId", stream));
    }
    if let Some(exclude_hostnames) = args.get("excludeHostnames").and_then(Value::as_array) {
        if !exclude_hostnames.is_empty() {
            let hostname_filters: Vec<Value> = exclude_hostnames
                .iter()
                .filter_map(|v| v.as_str())
                .filter(|s| !s.is_empty() && s.len() <= 255)
                .map(|hostname| exact("hostName", hostname))
                .collect();
            if !hostname_filters.is_empty() {
                filters.push(json!({
                    "notExpression": {
                        "orGroup": {
                            "expressions": hostname_filters
                        }
                    }
                }));
            }
        }
    }
    if filters.is_empty() {
        return Ok(());
    }
    if let Some(existing) = body.get("dimensionFilter") {
        filters.push(existing.clone());
    }
    body["dimensionFilter"] = json!({"andGroup":{"expressions":filters}});
    Ok(())
}
fn exact(field: &str, value: &str) -> Value {
    json!({"filter":{"fieldName":field,"stringFilter":{"matchType":"EXACT","value":value,"caseSensitive":true}}})
}
pub fn label(tool: &str, args: &Value) -> Value {
    let reporting = supported(tool) || tool == "analytics_realtime";
    let scoped = args.get("platform").is_some() || args.get("streamId").is_some();
    let excluded_hostnames = args.get("excludeHostnames")
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty());
    let filtered = args["request"].get("dimensionFilter").is_some()
        || args["request"]["requests"]
            .as_array()
            .is_some_and(|r| r.iter().any(|v| v.get("dimensionFilter").is_some()))
        || excluded_hostnames.is_some();
    json!({"propertyId":args["propertyId"],"accountId":args["accountId"],"platform":args.get("platform").cloned().unwrap_or(json!("all")),"streamId":args["streamId"],"excludedHostnames":excluded_hostnames,
        "kind":if !reporting {"property_or_account_evidence"} else if scoped {"selected_event_data"} else if filtered {"request_filtered"} else {"property_wide"},
        "note":if !reporting {"Property/account configuration, metadata or administrative evidence; no usage platform filter is applied."} else if scoped {"Selectors are ANDed with request filters. GA identity, compatibility and thresholding rules still apply; a platform is not a package-name selector."} else {"No platform or stream selected: this may combine Android, iOS and web. Custom request filters, if supplied, still apply."}})
}
