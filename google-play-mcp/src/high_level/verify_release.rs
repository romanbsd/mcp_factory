use mcp_factory_core::chrono::Utc;
use serde_json::{json, Value};

use super::client::EvidenceClient;
use super::project_status::{find_release, normalize_lifecycle, version_code_matches};
use super::quality::app;
use super::registry;

pub async fn report(client: &mut EvidenceClient<'_>, arguments: &Value) -> Value {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    let track = arguments["track"].as_str().unwrap_or_default();
    let version_code = arguments["versionCode"].as_i64().unwrap_or_default();
    let expected_hash = arguments["expectedChecksum"]
        .as_str()
        .map(|hash| hash.trim().to_ascii_lowercase());
    let expected_notes = arguments["releaseNotes"].as_array();
    let edit_id = arguments["editId"].as_str().filter(|id| !id.is_empty());

    let releases = client
        .call(
            "call-track-releases",
            "applications_tracks_releases_list",
            json!({"parent": format!("applications/{package}/tracks/{track}")}),
        )
        .await;
    let release = releases
        .as_ref()
        .and_then(|value| find_release(value, version_code));
    let lifecycle = release.map_or("unknown", |release| {
        normalize_lifecycle(
            release["releaseLifecycleState"]
                .as_str()
                .unwrap_or_default(),
        )
    });
    let release_found = match (&releases, release) {
        (None, _) => check(
            "unverifiable",
            "The track releases call failed; see sourceCalls.",
        ),
        (Some(_), None) => check(
            "mismatch",
            &format!("No active release on track '{track}' contains versionCode {version_code}."),
        ),
        (Some(_), Some(release)) => {
            json!({"result": "match", "releaseName": release["releaseName"]})
        }
    };

    let no_edit = "Checksums and release notes are exposed only inside an edit. Open one with edits_insert, pass its id as editId, then delete it with edits_delete.";
    let checksum = match (&expected_hash, edit_id) {
        (None, _) => check("not_requested", ""),
        (Some(_), None) => check("unverifiable", no_edit),
        (Some(expected), Some(edit_id)) => {
            let bundles = client
                .call(
                    "call-edit-bundles",
                    "edits_bundles_list",
                    json!({"packageName": package, "editId": edit_id}),
                )
                .await;
            compare_checksum(expected, version_code, bundles.as_ref())
        }
    };
    let release_notes = match (expected_notes, edit_id) {
        (None, _) => check("not_requested", ""),
        (Some(_), None) => check("unverifiable", no_edit),
        (Some(expected), Some(edit_id)) => {
            let edit_track = client
                .call(
                    "call-edit-track",
                    "edits_tracks_get",
                    json!({"packageName": package, "editId": edit_id, "track": track}),
                )
                .await;
            compare_notes(expected, version_code, edit_track.as_ref())
        }
    };

    let checks = json!({
        "releaseFound": release_found,
        "checksum": checksum,
        "releaseNotes": release_notes,
    });
    let all_match = checks
        .as_object()
        .into_iter()
        .flat_map(|checks| checks.values())
        .all(|check| matches!(check["result"].as_str(), Some("match" | "not_requested")));
    let summary = if all_match {
        format!("Release {version_code} on '{track}' matches every requested check; lifecycle is {lifecycle}.")
    } else {
        format!("Release {version_code} on '{track}' did not match every requested check; lifecycle is {lifecycle}.")
    };

    json!({
        "status": client.status(),
        "generatedAt": Utc::now().to_rfc3339(),
        "app": app(package),
        "summary": summary,
        "findings": [],
        "actions": [],
        "coverageGaps": registry::console_coverage_gaps(),
        "sourceCalls": client.source_calls(),
        "warnings": client.warnings(),
        "track": track,
        "versionCode": version_code,
        "lifecycle": lifecycle,
        "checks": checks,
        "allMatch": all_match,
    })
}

/// Checksums and notes come from the supplied edit, which includes any
/// uncommitted changes made in it, not only the committed release state.
const EDIT_SOURCE: &str = "Read from the supplied edit, including any uncommitted changes in it. Use a freshly opened edit to verify the committed release.";

fn check(result: &str, reason: &str) -> Value {
    if reason.is_empty() {
        json!({"result": result})
    } else {
        json!({"result": result, "reason": reason})
    }
}

fn compare_checksum(expected: &str, version_code: i64, bundles: Option<&Value>) -> Value {
    let Some(bundles) = bundles else {
        return check(
            "unverifiable",
            "The edit bundles call failed; see sourceCalls.",
        );
    };
    // 40 hex chars is SHA-1, anything else is compared as SHA-256.
    let field = if expected.len() == 40 {
        "sha1"
    } else {
        "sha256"
    };
    let bundle = bundles["bundles"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|bundle| version_code_matches(&bundle["versionCode"], version_code));
    let Some(bundle) = bundle else {
        return check(
            "mismatch",
            &format!("No bundle with versionCode {version_code} was found in the edit."),
        );
    };
    match bundle[field].as_str() {
        None => check(
            "unverifiable",
            &format!("The bundle with versionCode {version_code} has no {field} field."),
        ),
        Some(actual) => json!({
            "result": if actual.eq_ignore_ascii_case(expected) { "match" } else { "mismatch" },
            "algorithm": field,
            "expected": expected,
            "actual": actual.to_ascii_lowercase(),
            "source": EDIT_SOURCE,
        }),
    }
}

fn compare_notes(expected: &[Value], version_code: i64, edit_track: Option<&Value>) -> Value {
    let Some(edit_track) = edit_track else {
        return check(
            "unverifiable",
            "The edit track call failed; see sourceCalls.",
        );
    };
    let release = edit_track["releases"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|release| {
            release["versionCodes"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|code| version_code_matches(code, version_code))
        });
    let Some(release) = release else {
        return check(
            "mismatch",
            &format!("No release in the edit's track contains versionCode {version_code}."),
        );
    };
    let per_language: Vec<Value> = expected
        .iter()
        .map(|note| {
            let language = note["language"].as_str().unwrap_or_default();
            let expected_text = note["text"].as_str().unwrap_or_default().trim();
            let actual = release["releaseNotes"]
                .as_array()
                .into_iter()
                .flatten()
                .find(|actual| actual["language"].as_str() == Some(language))
                .and_then(|actual| actual["text"].as_str());
            json!({
                "language": language,
                "result": if actual.map(str::trim) == Some(expected_text) { "match" } else { "mismatch" },
                "expectedLength": expected_text.chars().count(),
                "actual": actual,
            })
        })
        .collect();
    let all = per_language.iter().all(|note| note["result"] == "match");
    json!({
        "result": if all { "match" } else { "mismatch" },
        "perLanguage": per_language,
        "source": EDIT_SOURCE,
    })
}
