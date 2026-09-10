use mcp_factory_core::chrono::Utc;
use serde_json::{json, Value};

use super::client::EvidenceClient;
use super::quality::app;
use super::registry;

pub async fn report(client: &mut EvidenceClient<'_>, arguments: &Value) -> Value {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    let message = arguments["message"].as_str().unwrap_or_default();
    let console_area = arguments["consoleArea"].as_str();
    let category = classify(message, console_area);
    let version_code = arguments.get("versionCode").and_then(Value::as_i64);
    let mut evidence = Vec::new();

    // Correlate a version code only when the console area is present and maps
    // to a known track. An absent, unknown, or unrecognized area must not
    // silently default to production and fabricate release evidence against a
    // track the message never referenced.
    if let (Some(version_code), Some(track)) = (version_code, track_from_area(console_area)) {
        let parent = format!("applications/{package}/tracks/{track}");
        if let Some(releases) = client
            .call(
                "call-console-message-release",
                "applications_tracks_releases_list",
                json!({"parent": parent}),
            )
            .await
        {
            let matched = releases["releases"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|release| {
                    release["activeArtifacts"]
                        .as_array()
                        .into_iter()
                        .flatten()
                        .any(|artifact| artifact["versionCode"].as_i64() == Some(version_code))
                });
            evidence.push(json!({
                "kind": "release_correlation",
                "versionCode": version_code,
                "track": track,
                "releaseFound": matched,
                "sourceCall": "call-console-message-release"
            }));
        }
    }

    let impact = if contains_any(message, &["error", "required", "blocked", "rejected"]) {
        "normally_blocking"
    } else if category == "unknown" {
        "context_dependent"
    } else {
        "normally_advisory"
    };
    let unknown = category == "unknown";
    let summary = if unknown {
        "The supplied Console message could not be classified safely; more page context is required."
    } else {
        "The supplied Console message was classified deterministically and correlated only with applicable API evidence."
    };
    let actions = vec![json!({
        "priority": 1,
        "title": if unknown { "Provide the Console page heading or screenshot" } else { "Review the finding in its named Console area" },
        "reason": if unknown { "Unknown wording must not be guessed." } else { "The public APIs do not expose the warning resolution state." },
        "safeDuringReview": true,
        "mutatesPlayState": false,
        "requiresConsole": true,
        "dependsOnFindingIds": ["supplied-console-message"]
    })];

    json!({
        "status": client.status(),
        "generatedAt": Utc::now().to_rfc3339(),
        "app": app(package),
        "summary": summary,
        "findings": [{
            "id": "supplied-console-message",
            "area": category,
            "severity": arguments["consoleSeverity"].as_str().unwrap_or("warning"),
            "state": if unknown { "unknown" } else { "inferred" },
            "title": "User-supplied Play Console message",
            "detail": "Classification does not prove that the Console message is resolved.",
            "inference": true,
            "rule": "google-play-console-message-v1",
            "evidenceStrength": if evidence.is_empty() { "none" } else { "low" },
            "evidenceRefs": evidence.iter().filter_map(|item| item["sourceCall"].as_str()).collect::<Vec<_>>(),
            "limitations": ["The exact Console state is not exposed by the public APIs."]
        }],
        "actions": actions,
        "coverageGaps": registry::console_coverage_gaps(),
        "sourceCalls": client.source_calls(),
        "warnings": client.warnings(),
        "userInput": {
            "message": message,
            "consoleArea": console_area,
            "consoleSeverity": arguments["consoleSeverity"].as_str(),
            "untrusted": true
        },
        "classification": {
            "category": category,
            "impact": impact,
            "ruleVersion": 1
        },
        "correlatedEvidence": evidence,
        "reviewImpact": "Read-only diagnostics are safe; changing an artifact or release may restart review."
    })
}

fn classify(message: &str, area: Option<&str>) -> &'static str {
    let text = format!(
        "{} {}",
        message.to_ascii_lowercase(),
        area.unwrap_or("").to_ascii_lowercase()
    );
    for (category, terms) in [
        ("data_safety", &["data safety", "data collection"][..]),
        ("target_api", &["target api", "api level"]),
        ("pre_launch", &["pre-launch", "pre launch"]),
        (
            "device_compatibility",
            &["device compatibility", "supported devices"],
        ),
        (
            "store_listing",
            &["store listing", "graphic asset", "screenshot"],
        ),
        (
            "app_content",
            &["app content", "content rating", "ads declaration"],
        ),
        (
            "artifact",
            &["app bundle", "apk", "mapping file", "debug symbols"],
        ),
        ("quality", &["crash", "anr", "android vitals"]),
        ("policy", &["policy", "permissions declaration"]),
    ] {
        if terms.iter().any(|term| text.contains(term)) {
            return category;
        }
    }
    "unknown"
}

fn track_from_area(area: Option<&str>) -> Option<&'static str> {
    let area = area?.to_ascii_lowercase();
    if area.contains("production") {
        Some("production")
    } else if area.contains("open testing") {
        Some("beta")
    } else if area.contains("closed testing") {
        Some("alpha")
    } else if area.contains("internal") {
        Some("internal")
    } else {
        None
    }
}

fn contains_any(text: &str, needles: &[&str]) -> bool {
    let text = text.to_ascii_lowercase();
    needles.iter().any(|needle| text.contains(needle))
}
