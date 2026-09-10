use mcp_factory_core::chrono::Utc;
use serde_json::{json, Value};

use super::client::EvidenceClient;
use super::quality::app;
use super::registry;

pub async fn report(client: &mut EvidenceClient<'_>, arguments: &Value) -> Value {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    if !arguments["probe"].as_bool().unwrap_or(true) {
        return envelope(
            "complete",
            app(package),
            "Capability metadata was returned without live API probes.".to_string(),
            json!([]),
            json!([]),
            json!({"androidPublisher": "not_probed", "playDeveloperReporting": "not_probed"}),
            json!({"androidPublisher": "not_probed", "playDeveloperReporting": "not_probed"}),
            json!({"androidPublisher": {"condition": "not_probed"}, "playDeveloperReporting": {"condition": "not_probed"}}),
            Value::Null,
            json!([]),
            json!([{"message": "Live access was not checked because probe=false."}]),
        );
    }
    let (pages, complete) = client
        .call_pages(
            "call-accessible-apps",
            "apps_search",
            json!({"pageSize": 1000}),
            &["pageToken"],
            &["nextPageToken"],
        )
        .await;
    let resolved = pages
        .iter()
        .flat_map(|page| page["apps"].as_array().into_iter().flatten())
        .find(|candidate| candidate["packageName"] == package)
        .cloned();

    let publisher = client
        .call(
            "call-publisher-access",
            "reviews_list",
            json!({"packageName": package, "maxResults": 1}),
        )
        .await;
    let source_calls = client.source_calls();
    let reporting_available = method_succeeded(&source_calls, "apps_search");
    let reporting_app_access = resolved.is_some();
    // A missing package proves inaccessibility only when every page was read. If
    // apps_search paginated but a later page failed, the package may sit on an
    // unretrieved page, so its access is unknown, not confirmed inaccessible.
    let reporting_app_unknown = !reporting_app_access && reporting_available && !complete;
    let publisher_available = publisher.is_some();
    let mut findings = Vec::new();
    if !reporting_app_access {
        let (id, title, detail, state, strength, limitations) = if !reporting_available {
            (
                "reporting-api-unavailable",
                "Play Developer Reporting API is unavailable",
                "The Reporting API request failed.",
                "confirmed",
                "high",
                json!([]),
            )
        } else if reporting_app_unknown {
            (
                "reporting-app-access-unknown",
                "App accessibility through Play Developer Reporting is unknown",
                "The API responded, but apps_search pagination did not complete, so the package's absence is not confirmed.",
                "unknown",
                "low",
                json!([
                    "Pagination did not complete; the package may be on an unretrieved page."
                ]),
            )
        } else {
            (
                "reporting-app-inaccessible",
                "The app is not accessible through Play Developer Reporting",
                "The API responded and pagination completed, but apps_search did not return the requested package.",
                "confirmed",
                "high",
                json!([]),
            )
        };
        findings.push(json!({
            "id": id,
            "area": "capability",
            "severity": "action_required",
            "state": state,
            "title": title,
            "detail": detail,
            "inference": false,
            "evidenceStrength": strength,
            "evidenceRefs": ["call-accessible-apps-page-1"],
            "limitations": limitations
        }));
    }
    if !publisher_available {
        findings.push(json!({
            "id": "publisher-api-unavailable",
            "area": "capability",
            "severity": "action_required",
            "state": "confirmed",
            "title": "Android Publisher access is unavailable",
            "detail": "The read-only reviews probe failed.",
            "inference": false,
            "evidenceStrength": "high",
            "evidenceRefs": ["call-publisher-access"],
            "limitations": []
        }));
    }

    let activation_url = find_activation_url(&source_calls);
    let actions = activation_url
        .map(|url| {
            vec![json!({
                "priority": 1,
                "title": "Enable the reported Google API",
                "reason": "Google returned a service activation URL.",
                "safeDuringReview": true,
                "mutatesPlayState": false,
                "requiresConsole": true,
                "activationUrl": url,
                "dependsOnFindingIds": ["reporting-api-unavailable"]
            })]
        })
        .unwrap_or_default();
    let application = resolved.unwrap_or_else(|| app(package));
    let summary = match (publisher_available, reporting_available, reporting_app_access) {
        (true, true, true) => "Publisher and Developer Reporting read access are available.",
        (true, true, false) if reporting_app_unknown => "Both APIs responded, but pagination did not complete, so app accessibility through Developer Reporting is unknown.",
        (true, true, false) => "Both APIs responded, but the requested app was not returned by Developer Reporting.",
        (true, false, _) => "Publisher access is available; Developer Reporting is unavailable.",
        (false, true, true) => "Developer Reporting is available; Publisher access is unavailable.",
        (false, true, false) if reporting_app_unknown => "Developer Reporting responded but pagination did not complete, so app accessibility is unknown; Publisher access is unavailable.",
        (false, true, false) => "Developer Reporting responded without the requested app; Publisher access is unavailable.",
        (false, false, _) => "Neither Google Play API produced material evidence.",
    };

    envelope(
        client.status(),
        application,
        summary.to_string(),
        json!(findings),
        json!(actions),
        json!({
            "androidPublisher": if publisher_available { "available" } else { "unavailable" },
            "playDeveloperReporting": if reporting_available { "available" } else { "unavailable" },
        }),
        json!({
            "androidPublisher": if publisher_available { "available" } else { "unavailable" },
            "playDeveloperReporting": if reporting_app_access { "available" } else if reporting_app_unknown { "unknown" } else if reporting_available { "package_not_returned" } else { "unavailable" },
        }),
        json!({
            "androidPublisher": diagnose_method(&source_calls, "reviews_list"),
            "playDeveloperReporting": diagnose_method(&source_calls, "apps_search"),
        }),
        json!(complete),
        json!(source_calls),
        json!(client.warnings()),
    )
}

#[allow(clippy::too_many_arguments)]
fn envelope(
    status: &str,
    application: Value,
    summary: String,
    findings: Value,
    actions: Value,
    api_availability: Value,
    application_access: Value,
    api_diagnostics: Value,
    pagination_complete: Value,
    source_calls: Value,
    warnings: Value,
) -> Value {
    json!({
        "status": status,
        "generatedAt": Utc::now().to_rfc3339(),
        "app": application,
        "summary": summary,
        "findings": findings,
        "actions": actions,
        "coverageGaps": registry::console_coverage_gaps(),
        "sourceCalls": source_calls,
        "warnings": warnings,
        "apiAvailability": api_availability,
        "applicationAccess": application_access,
        "apiDiagnostics": api_diagnostics,
        "paginationComplete": pagination_complete,
        "capabilityRegistry": registry::capabilities_json(),
        "readOnlyMethodAllowlist": registry::ALLOWED_LOW_LEVEL_METHODS,
    })
}

fn method_succeeded(source_calls: &[Value], method: &str) -> bool {
    source_calls
        .iter()
        .any(|call| call["method"] == method && call["resultState"] == "success")
}

fn diagnose_method(source_calls: &[Value], method: &str) -> Value {
    let Some(call) = source_calls.iter().rev().find(|call| call["method"] == method) else {
        return json!({"condition": "not_probed"});
    };
    if call["resultState"] == "success" {
        return json!({"condition": "available"});
    }
    let error = &call["error"];
    let text = error.to_string();
    let status = error["status"].as_u64();
    let condition = if status == Some(401) || text.contains("UNAUTHENTICATED") {
        "authentication_required"
    } else if text.contains("SERVICE_DISABLED") {
        "service_disabled"
    } else if status == Some(403) || text.contains("PERMISSION_DENIED") {
        "permission_denied"
    } else if status == Some(429) || text.contains("RESOURCE_EXHAUSTED") {
        "quota_exhausted"
    } else if matches!(status, Some(500 | 502 | 503 | 504)) || text.contains("UNAVAILABLE") {
        "transient_unavailable"
    } else {
        "unavailable"
    };
    json!({"condition": condition, "status": status})
}

fn find_activation_url(values: &[Value]) -> Option<String> {
    values.iter().find_map(find_url)
}

fn find_url(value: &Value) -> Option<String> {
    match value {
        Value::Object(object) => object.iter().find_map(|(key, value)| {
            if key == "activationUrl" {
                value.as_str().map(str::to_string)
            } else {
                find_url(value)
            }
        }),
        Value::Array(values) => values.iter().find_map(find_url),
        _ => None,
    }
}
