use std::collections::HashMap;

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
            // Distinct source tracks can resolve to the same Publisher track id
            // (e.g. several open-testing tracks all map to the default "beta"),
            // so calls are memoized per id rather than reissued per source track
            // to avoid burning through the tight "Listing releases" quota.
            let mut release_calls: HashMap<String, Option<Value>> = HashMap::new();
            for track in filters["tracks"].as_array().into_iter().flatten() {
                let (track_id, inferred) = normalize_track(track);
                let direct = if let Some(cached) = release_calls.get(&track_id) {
                    cached.clone()
                } else {
                    let result = client
                        .call(
                            &format!("call-{track_id}-releases"),
                            "applications_tracks_releases_list",
                            json!({"parent": format!("applications/{package}/tracks/{track_id}")}),
                        )
                        .await;
                    release_calls.insert(track_id.clone(), result.clone());
                    result
                };
                tracks.push(normalize_release_track(
                    track_id,
                    inferred,
                    track,
                    direct.as_ref(),
                ));
            }
        }
        // The reporting API does not list every Publisher track (custom testing
        // tracks, or none at all if its call failed), so a requested track it
        // omitted is queried directly; its serving state stays unknown. Only a
        // complete track + versionCode request can use it, so a lone track is
        // not worth a call against the tight releases quota.
        if let (Some(track_id), Some(_)) = (
            arguments["track"].as_str(),
            arguments["versionCode"].as_i64(),
        ) {
            if !tracks.iter().any(|entry| entry["track"] == track_id) {
                let direct = client
                    .call(
                        &format!("call-{track_id}-releases"),
                        "applications_tracks_releases_list",
                        json!({"parent": format!("applications/{package}/tracks/{track_id}")}),
                    )
                    .await;
                let mut entry = normalize_release_track(
                    track_id.to_string(),
                    false,
                    &json!({}),
                    direct.as_ref(),
                );
                entry["servingUnknown"] = json!(true);
                tracks.push(entry);
            }
        }
    }

    // quality::report and the reviews pagination below are independent
    // read-only queries, so they run concurrently rather than one after the
    // other — a shared reference is safe since `EvidenceClient`'s bookkeeping
    // lives behind a `Mutex`.
    let client: &EvidenceClient<'_> = client;
    let want_quality = wants("quality");
    let want_reviews = wants("reviews");
    let (quality_report, reviews) = tokio::join!(
        async {
            if want_quality {
                Some(quality::report(client, arguments).await)
            } else {
                None
            }
        },
        async {
            if want_reviews {
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
            }
        }
    );
    let production = tracks.iter().find(|track| track["track"] == "production");
    let lifecycle = production
        .and_then(|track| track["releases"].as_array())
        .and_then(|releases| releases.first())
        .and_then(|release| release["lifecycle"].as_str())
        .unwrap_or("unknown");
    // A production entry added by direct lookup has no Reporting API serving
    // data, so an empty list there means unknown, not "not serving".
    let production_serving_known = production.is_some_and(|track| track["servingUnknown"] != true);
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

    // apprecovery_list requires a real target versionCode; the API rejects an
    // omitted/defaulted 0, so the lookup only runs once a serving production
    // version is known (computed above).
    let recoveries = if wants("recoveries") {
        match production_serving.first() {
            Some(version_code) => client
                .call(
                    "call-recovery-actions",
                    "apprecovery_list",
                    json!({"packageName": package, "versionCode": version_code}),
                )
                .await
                .map(|value| {
                    json!({
                        "count": value["recoveryActions"].as_array().map_or(0, Vec::len),
                        "actions": value["recoveryActions"]
                    })
                }),
            None => Some(json!({
                "count": 0,
                "actions": [],
                "assessment": "No production serving version code is known yet; recovery actions require a targeted version."
            })),
        }
    } else {
        None
    };

    let mut findings = Vec::new();
    if lifecycle == "under_review" && production_serving_known && production_serving.is_empty() {
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

    let requested = match (
        arguments["track"].as_str(),
        arguments["versionCode"].as_i64(),
    ) {
        (Some(track), Some(version_code)) if wants("releases") => {
            Some(requested_release(&tracks, track, version_code))
        }
        (Some(_), None) | (None, Some(_)) if wants("releases") => Some(json!({
            "track": arguments["track"],
            "versionCode": arguments["versionCode"],
            "found": null,
            "lifecycle": "unknown",
            "unresolved": ["Both track and versionCode are required to report a specific release."],
        })),
        _ => None,
    };
    if let Some(requested) = &requested {
        let requested_lifecycle = requested["lifecycle"].as_str().unwrap_or("unknown");
        // Anything short of published or in review (rejected, draft, approved
        // but unpublished, missing, or unreadable) means the build has not shipped.
        if !matches!(requested_lifecycle, "published" | "under_review") {
            let title = match requested["found"].as_bool() {
                Some(true) => format!(
                    "Requested release {} is {requested_lifecycle}",
                    requested["versionCode"]
                ),
                Some(false) => format!(
                    "Requested release {} was not found on '{}'",
                    requested["versionCode"],
                    requested["track"].as_str().unwrap_or_default()
                ),
                None => "Requested release could not be checked".to_string(),
            };
            findings.push(json!({
                "id": "requested-release-not-live",
                "area": "release",
                "severity": "warning",
                "state": if requested["found"] == true { "confirmed" } else { "unknown" },
                "title": title,
                "detail": "Other releases on this track may still be serving; they are not the requested build.",
                "inference": false,
                "evidenceStrength": if requested["unresolved"].as_array().is_some_and(Vec::is_empty) { "high" } else { "low" },
                "evidenceRefs": [format!("call-{}-releases", requested["track"].as_str().unwrap_or_default())],
                "limitations": requested["unresolved"]
            }));
        }
    }

    let summary = if let Some(requested) = requested.as_ref().filter(|r| r["found"].is_null()) {
        format!(
            "The requested release could not be checked: {}",
            requested["unresolved"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(" ")
        )
    } else if let Some(requested) = &requested {
        let others = requested["otherServingVersionCodes"]
            .as_array()
            .filter(|codes| !codes.is_empty())
            .map(|codes| {
                format!(
                    "; '{}' still serves {}",
                    requested["track"].as_str().unwrap_or_default(),
                    Value::Array(codes.clone())
                )
            })
            .unwrap_or_default();
        format!(
            "Requested release {} on '{}' is {}{}{}.",
            requested["versionCode"],
            requested["track"].as_str().unwrap_or_default(),
            requested["lifecycle"].as_str().unwrap_or("unknown"),
            if requested["servingOnTrack"] == true {
                " and serving"
            } else {
                ""
            },
            others
        )
    } else if lifecycle == "under_review" {
        "The latest production release is under review and is not treated as published unless serving evidence exists.".to_string()
    } else if lifecycle == "rejected" {
        "The latest production release was rejected; any serving production version is an older release.".to_string()
    } else if lifecycle == "published" && !production_serving.is_empty() {
        "The latest production release is published and serving.".to_string()
    } else {
        "Release and quality evidence were collected; unsupported Console checks remain explicit."
            .to_string()
    };
    let requested_lifecycle = requested
        .as_ref()
        .map(|requested| requested["lifecycle"].as_str().unwrap_or("unknown"));
    let overall = if matches!(requested_lifecycle, Some("under_review"))
        || (requested_lifecycle.is_none() && lifecycle == "under_review")
    {
        "waiting_on_google"
    } else if requested_lifecycle.unwrap_or(lifecycle) == "rejected"
        || findings
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
        "sourceCalls": client.source_calls(),
        "warnings": client.warnings(),
        "requestedRelease": requested,
        "releaseState": lifecycle,
        "releaseStateScope": "app_production_latest",
        "publicServingVersionCodes": if production_serving_known { json!(production_serving) } else { Value::Null },
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
        "releasesCallFailed": direct.is_none(),
        "type": track["type"],
        "displayName": track["displayName"],
        "servingVersionCodes": serving,
        "releases": releases,
    })
}

pub(super) fn normalize_lifecycle(state: &str) -> &'static str {
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

/// Version codes arrive as integers from the releases API and as strings from
/// the reporting and edits APIs, so both shapes are compared.
pub(super) fn version_code_matches(value: &Value, version_code: i64) -> bool {
    value.as_i64() == Some(version_code)
        || value.as_str().and_then(|code| code.parse().ok()) == Some(version_code)
}

/// Finds the release in an `applications_tracks_releases_list` response whose
/// active artifacts include `version_code`.
pub(super) fn find_release(releases: &Value, version_code: i64) -> Option<&Value> {
    releases["releases"].as_array()?.iter().find(|release| {
        release["activeArtifacts"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|artifact| version_code_matches(&artifact["versionCode"], version_code))
    })
}

/// Status of one caller-named release, so an older release still serving on
/// the same track is never reported as the outcome of the requested build.
fn requested_release(tracks: &[Value], track: &str, version_code: i64) -> Value {
    let mut unresolved = Vec::new();
    let entry = tracks.iter().find(|entry| entry["track"] == track);
    let lookup_failed = entry.is_none_or(|entry| entry["releasesCallFailed"] == true);
    if lookup_failed {
        unresolved.push(format!(
            "The releases call for '{track}' failed, so the requested release could not be checked."
        ));
    }
    let serving_unknown = entry.is_some_and(|entry| entry["servingUnknown"] == true);
    if serving_unknown {
        unresolved.push(format!(
            "'{track}' is not listed by the Reporting API, so its serving state is unknown."
        ));
    }
    if entry.is_some_and(|entry| entry["trackIdInferred"] == true) {
        unresolved.push(format!("Track id '{track}' was inferred from the reporting API and may not be the Publisher track id."));
    }
    let release = entry
        .and_then(|entry| entry["releases"].as_array())
        .into_iter()
        .flatten()
        .find(|release| {
            release["versionCodes"]
                .as_array()
                .into_iter()
                .flatten()
                .any(|code| version_code_matches(code, version_code))
        });
    if !lookup_failed && release.is_none() {
        unresolved.push(format!(
            "No active release on '{track}' contains versionCode {version_code}."
        ));
    }
    let serving_on_track = entry
        .and_then(|entry| entry["servingVersionCodes"].as_array())
        .into_iter()
        .flatten()
        .any(|code| version_code_matches(code, version_code));
    let other_serving: Vec<Value> = entry
        .and_then(|entry| entry["servingVersionCodes"].as_array())
        .into_iter()
        .flatten()
        .filter(|code| !version_code_matches(code, version_code))
        .cloned()
        .collect();
    json!({
        "track": track,
        "versionCode": version_code,
        "found": if lookup_failed { Value::Null } else { json!(release.is_some()) },
        "releaseName": release.map(|release| release["name"].clone()),
        "lifecycle": release.and_then(|release| release["lifecycle"].as_str()).unwrap_or("unknown"),
        "servingOnTrack": if serving_unknown { Value::Null } else { json!(serving_on_track) },
        "otherServingVersionCodes": other_serving,
        "unresolved": unresolved,
    })
}
