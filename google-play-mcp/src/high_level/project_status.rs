use mcp_factory_core::chrono::Utc;
use serde_json::{json, Value};

use super::client::EvidenceClient;
use super::quality;
use super::registry;

pub async fn report(client: &mut EvidenceClient<'_>, arguments: &Value) -> Value {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    let include = arguments["include"].as_array();
    let wants = |area: &str| {
        include.is_none_or(|items| {
            items.is_empty() || items.iter().any(|item| item.as_str() == Some(area))
        })
    };

    let mut tracks = Vec::new();
    if wants("releases") {
        let filters = client
            .call(
                "call-release-filter-options",
                "apps_fetchReleaseFilterOptions",
                json!({"name": format!("apps/{package}")}),
            )
            .await;
        if let Some(filters) = &filters {
            for track in filters["tracks"].as_array().into_iter().flatten() {
                let (track_id, inferred) = normalize_track(track);
                let direct = client
                    .call(
                        &format!("call-{track_id}-releases"),
                        "applications_tracks_releases_list",
                        json!({"parent": format!("applications/{package}/tracks/{track_id}")}),
                    )
                    .await;
                tracks.push(normalize_release_track(
                    track_id,
                    inferred,
                    track,
                    direct.as_ref(),
                ));
            }
        }
    }

    let quality_report = if wants("quality") {
        Some(quality::report(client, arguments).await)
    } else {
        None
    };
    let reviews = if wants("reviews") {
        let (pages, complete) = client
            .call_pages(
                "call-reviews",
                "reviews_list",
                json!({"packageName": package, "maxResults": 100}),
                &["token"],
                &["tokenPagination", "nextPageToken"],
            )
            .await;
        Some(json!({
            "returnedReviews": pages.iter().map(|page| page["reviews"].as_array().map_or(0, Vec::len)).sum::<usize>(),
            "paginationComplete": complete,
            "privacy": "Review text and reviewer identity are omitted from this summary."
        }))
    } else {
        None
    };
    let recoveries = if wants("recoveries") {
        client
            .call(
                "call-recovery-actions",
                "apprecovery_list",
                json!({"packageName": package}),
            )
            .await
            .map(|value| {
                json!({
                    "count": value["recoveryActions"].as_array().map_or(0, Vec::len),
                    "actions": value["recoveryActions"]
                })
            })
    } else {
        None
    };

    let production = tracks.iter().find(|track| track["track"] == "production");
    let lifecycle = production
        .and_then(|track| track["releases"].as_array())
        .and_then(|releases| releases.first())
        .and_then(|release| release["lifecycle"].as_str())
        .unwrap_or("unknown");
    let production_serving: Vec<Value> = production
        .and_then(|track| track["servingVersionCodes"].as_array())
        .cloned()
        .unwrap_or_default();
    let testing_serving: Vec<Value> = tracks
        .iter()
        .filter(|track| track["track"] != "production")
        .flat_map(|track| {
            track["servingVersionCodes"]
                .as_array()
                .into_iter()
                .flatten()
                .cloned()
        })
        .collect();

    let mut findings = Vec::new();
    if lifecycle == "under_review" && production_serving.is_empty() {
        findings.push(json!({
            "id": "production-release-in-review",
            "area": "release",
            "severity": "info",
            "state": "confirmed",
            "title": "Production release is in review",
            "detail": "A production release is present but is not serving publicly.",
            "inference": false,
            "evidenceStrength": "high",
            "evidenceRefs": ["call-production-releases", "call-release-filter-options"],
            "limitations": []
        }));
    }
    let quality_metrics = quality_report
        .as_ref()
        .and_then(|report| report["metrics"].as_array())
        .cloned()
        .unwrap_or_default();
    for metric in &quality_metrics {
        if metric["dataState"] == "observed_nonzero" {
            findings.push(json!({
                "id": format!("quality-{}-{}", metric["metric"].as_str().unwrap_or("metric"), metric["cohort"].as_str().unwrap_or("cohort")),
                "area": "quality",
                "severity": "warning",
                "state": "confirmed",
                "title": "A non-zero Android vitals observation was returned",
                "detail": metric["assessment"],
                "inference": false,
                "evidenceStrength": metric["evidenceStrength"],
                "evidenceRefs": [],
                "limitations": []
            }));
        }
    }

    let summary = if lifecycle == "under_review" {
        "Production is under review and is not treated as published unless serving evidence exists."
    } else if lifecycle == "published" && !production_serving.is_empty() {
        "A production release is published and serving."
    } else {
        "Release and quality evidence were collected; unsupported Console checks remain explicit."
    };
    let overall = if lifecycle == "under_review" {
        "waiting_on_google"
    } else if findings
        .iter()
        .any(|finding| finding["severity"] == "warning")
    {
        "attention_recommended"
    } else {
        "bounded_no_api_visible_blocker"
    };

    json!({
        "status": client.status(),
        "generatedAt": Utc::now().to_rfc3339(),
        "app": quality::app(package),
        "summary": summary,
        "findings": findings,
        "actions": review_safe_actions(lifecycle),
        "coverageGaps": registry::console_coverage_gaps(),
        "sourceCalls": client.source_calls.clone(),
        "warnings": client.warnings.clone(),
        "releaseState": lifecycle,
        "publicServingVersionCodes": production_serving,
        "testingServingVersionCodes": testing_serving,
        "overallAssessment": overall,
        "evidenceStrength": if client.status() == "complete" { "medium" } else { "low" },
        "tracks": tracks,
        "quality": quality_report.as_ref().map(|report| json!({
            "metrics": report["metrics"],
            "errorEvidence": report["errorEvidence"],
            "anomalyEvidence": report["anomalyEvidence"],
            "requestedLookbackDays": report["requestedLookbackDays"],
        })),
        "reviews": reviews,
        "recoveries": recoveries,
    })
}

/// Maps a Play Developer Reporting track (which exposes only `type` and
/// `displayName`, never the Android Publisher track id) to a Publisher track id
/// for the releases fetch. Returns `(track_id, inferred)`: `production` and
/// `internal` are unambiguous, but `open`/`closed` map to the *default* `beta`/
/// `alpha` ids — wrong for custom-named testing tracks, whose real id the
/// reporting API does not expose. `inferred` marks that best-effort guess so an
/// empty releases result isn't presented as fact.
fn normalize_track(track: &Value) -> (String, bool) {
    let kind = track["type"]
        .as_str()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if kind.contains("production") {
        ("production".to_string(), false)
    } else if kind.contains("internal") {
        ("internal".to_string(), false)
    } else if kind.contains("open") {
        ("beta".to_string(), true)
    } else if kind.contains("closed") {
        track["displayName"]
            .as_str()
            .filter(|track_id| !track_id.is_empty())
            .map_or_else(
                || ("alpha".to_string(), true),
                |track_id| (track_id.to_string(), false),
            )
    } else {
        (
            track["displayName"]
                .as_str()
                .unwrap_or("unknown")
                .to_ascii_lowercase()
                .replace(' ', "-"),
            true,
        )
    }
}

fn normalize_release_track(
    track_id: String,
    inferred: bool,
    track: &Value,
    direct: Option<&Value>,
) -> Value {
    let serving: Vec<Value> = track["servingReleases"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|release| {
            release["versionCodes"]
                .as_array()
                .into_iter()
                .flatten()
                .cloned()
        })
        .collect();
    let releases: Vec<Value> = direct
        .and_then(|value| value["releases"].as_array())
        .into_iter()
        .flatten()
        .map(|release| {
            let state = release["releaseLifecycleState"].as_str().unwrap_or_default();
            json!({
                "name": release["releaseName"],
                "lifecycle": normalize_lifecycle(state),
                "rawLifecycle": state,
                "versionCodes": release["activeArtifacts"].as_array().into_iter().flatten().filter_map(|artifact| artifact.get("versionCode")).cloned().collect::<Vec<_>>(),
            })
        })
        .collect();
    // A guessed (inferred) id with no releases may be a wrong-parent miss for a
    // custom testing track, not a genuinely empty track — flag it rather than
    // silently reporting no releases.
    let releases_unresolved = inferred && releases.is_empty();
    json!({
        "track": track_id,
        "trackIdInferred": inferred,
        "releasesUnresolved": releases_unresolved,
        "type": track["type"],
        "displayName": track["displayName"],
        "servingVersionCodes": serving,
        "releases": releases,
    })
}

fn normalize_lifecycle(state: &str) -> &'static str {
    match state {
        "RELEASE_LIFECYCLE_STATE_DRAFT" => "draft",
        "RELEASE_LIFECYCLE_STATE_NOT_SENT_FOR_REVIEW" => "not_sent_for_review",
        "RELEASE_LIFECYCLE_STATE_IN_REVIEW" => "under_review",
        "RELEASE_LIFECYCLE_STATE_APPROVED_NOT_PUBLISHED" => "approved_not_published",
        "RELEASE_LIFECYCLE_STATE_NOT_APPROVED" => "rejected",
        "RELEASE_LIFECYCLE_STATE_PUBLISHED" => "published",
        _ => "unknown",
    }
}

fn review_safe_actions(lifecycle: &str) -> Vec<Value> {
    if lifecycle == "under_review" {
        vec![json!({
            "priority": 1,
            "title": "Wait for the current production review",
            "reason": "The release is already in review; no API-visible blocker was established.",
            "safeDuringReview": true,
            "mutatesPlayState": false,
            "requiresConsole": false,
            "dependsOnFindingIds": ["production-release-in-review"]
        })]
    } else {
        Vec::new()
    }
}
