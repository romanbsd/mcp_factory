use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

use mcp_factory_core::{async_trait, ProxyError, ReadOnlyToolInvoker, ToolBody, ToolResult};
use serde_json::{json, Value};

use super::{build_tools, registry};

#[derive(Default)]
struct FakeInvoker {
    responses: Mutex<HashMap<String, VecDeque<ToolResult>>>,
    calls: Mutex<Vec<(String, Value)>>,
}

impl FakeInvoker {
    fn with(self, method: &str, responses: Vec<ToolResult>) -> Self {
        self.responses
            .lock()
            .unwrap()
            .insert(method.to_string(), responses.into());
        self
    }

    fn calls(&self) -> Vec<(String, Value)> {
        self.calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl ReadOnlyToolInvoker for FakeInvoker {
    async fn invoke_read_only(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        self.calls
            .lock()
            .unwrap()
            .push((name.to_string(), arguments));
        self.responses
            .lock()
            .unwrap()
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .ok_or_else(|| ProxyError::Other(format!("no fake response for {name}")))
    }
}

fn ok(value: Value) -> ToolResult {
    ToolResult::text(value.to_string()).with_structured(Some(value))
}

fn error(status: u64, message: &str, detail: Value) -> ToolResult {
    ToolResult {
        body: ToolBody::Text(message.to_string()),
        structured: Some(json!({"status": status, "problem": detail})),
        meta: Default::default(),
        is_error: true,
    }
}

async fn run(name: &str, arguments: Value, invoker: &FakeInvoker) -> Value {
    let tool = build_tools()
        .into_iter()
        .find(|tool| tool.name == name)
        .unwrap();
    tool.handler
        .call(invoker, arguments)
        .await
        .unwrap()
        .structured
        .unwrap()
}

fn freshness() -> ToolResult {
    ok(json!({
        "freshnessInfo": {"freshnesses": [{
            "aggregationPeriod": "DAILY",
            "latestEndTime": {"year": 2026, "month": 9, "day": 3, "timeZone": {"id": "America/Los_Angeles"}}
        }]}
    }))
}

fn zero_row(users: &str, upper: &str) -> Value {
    json!({
        "metrics": [
            {"metric": "anrRate", "decimalValue": {"value": "0"}, "decimalValueConfidenceInterval": {"upperBound": {"value": upper}}},
            {"metric": "distinctUsers", "decimalValue": {"value": users}}
        ]
    })
}

#[test]
fn exposes_exactly_the_four_read_only_p0_tools() {
    let tools = build_tools();
    assert_eq!(
        tools
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>(),
        [
            "report_capabilities",
            "report_project_status",
            "report_quality_health",
            "report_explain_console_message",
            "report_release_verification"
        ]
    );
    assert!(tools
        .iter()
        .all(|tool| tool.hints.read_only == Some(true) && tool.hints.destructive == Some(false)));
}

#[tokio::test]
async fn capability_metadata_can_be_read_without_live_probes() {
    let invoker = FakeInvoker::default();
    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": false}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "complete");
    assert_eq!(report["apiAvailability"]["androidPublisher"], "not_probed");
    assert!(invoker.calls().is_empty());
}

#[tokio::test]
async fn project_status_keeps_in_review_production_distinct_from_serving_test_track() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "PRODUCTION", "displayName": "Production", "servingReleases": []},
                {"type": "CLOSED_TESTING", "displayName": "Closed testing", "servingReleases": [{"displayName": "1.0.0", "versionCodes": ["11"]}]}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![
                ok(json!({"releases": [{
                    "releaseName": "1.0.0",
                    "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_IN_REVIEW",
                    "activeArtifacts": [{"versionCode": 11}]
                }]})),
                ok(json!({"releases": [{
                    "releaseName": "1.0.0",
                    "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED",
                    "activeArtifacts": [{"versionCode": 11}]
                }]})),
            ],
        );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"]}),
        &invoker,
    )
    .await;

    assert_eq!(report["releaseState"], "under_review");
    assert_eq!(report["publicServingVersionCodes"], json!([]));
    assert_eq!(report["testingServingVersionCodes"], json!(["11"]));
    assert_eq!(report["overallAssessment"], "waiting_on_google");
    assert_eq!(report["findings"][0]["id"], "production-release-in-review");
}

#[tokio::test]
async fn quality_distinguishes_no_data_from_sparse_observed_zero() {
    let sparse_rows = vec![zero_row("10", "0.9602"); 7];
    let invoker = FakeInvoker::default()
        .with("vitals_anrrate_get", vec![freshness(), freshness()])
        .with(
            "vitals_anrrate_query",
            vec![ok(json!({"rows": []})), ok(json!({"rows": sparse_rows}))],
        );

    let report = run(
        "report_quality_health",
        json!({
            "packageName": "org.example.app",
            "lookbackDays": 90,
            "cohorts": ["OS_PUBLIC", "APP_TESTERS"],
            "metrics": ["anr_rate"],
            "includeIssues": false,
            "includeAnomalies": false
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["metrics"][0]["dataState"], "no_data");
    assert!(report["metrics"][0]["pointEstimateMaximum"].is_null());
    assert_eq!(report["metrics"][1]["dataState"], "observed_zero");
    assert_eq!(report["metrics"][1]["observedDays"], 7);
    assert_eq!(report["metrics"][1]["maxDailyDistinctUsers"], 10.0);
    assert_eq!(
        report["metrics"][1]["confidenceIntervalUpperMaximum"],
        0.9602
    );
    assert_eq!(report["metrics"][1]["evidenceStrength"], "low");
    assert!(report["metrics"][1]["assessment"]
        .as_str()
        .unwrap()
        .contains("too small"));
}

#[tokio::test]
async fn invalid_metric_combination_is_prevented_without_an_upstream_probe() {
    let invoker = FakeInvoker::default();
    let report = run(
        "report_quality_health",
        json!({
            "packageName": "org.example.app",
            "cohorts": ["APP_TESTERS"],
            "metrics": ["slow_start_rate"],
            "includeIssues": false,
            "includeAnomalies": false
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["metrics"][0]["dataState"], "unsupported");
    assert!(invoker.calls().is_empty());
}

#[test]
fn capability_registry_preserves_required_dimensions_and_exact_metric_names() {
    let capabilities = registry::capabilities_json();
    let slow_start = capabilities
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["metric"] == "slow_start_rate")
        .unwrap();
    let stuck_wakelock = capabilities
        .as_array()
        .unwrap()
        .iter()
        .find(|item| item["metric"] == "stuck_background_wakelock_rate")
        .unwrap();

    assert_eq!(slow_start["requiredDimensions"], json!(["startType"]));
    assert_eq!(stuck_wakelock["metrics"][0], "stuckBgWakelockRate");
}

#[tokio::test]
async fn capability_probe_retries_service_propagation_and_consumes_all_pages() {
    let disabled = error(
        403,
        "SERVICE_DISABLED",
        json!({"activationUrl": "https://console.example/enable"}),
    );
    let invoker = FakeInvoker::default()
        .with(
            "apps_search",
            vec![
                disabled.clone(),
                disabled,
                ok(json!({"apps": [], "nextPageToken": "next"})),
                ok(json!({"apps": [{"packageName": "org.example.app", "name": "apps/123", "displayName": "Example"}]})),
            ],
        )
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "complete");
    assert_eq!(
        report["apiAvailability"]["playDeveloperReporting"],
        "available"
    );
    assert_eq!(report["paginationComplete"], true);
    assert_eq!(report["sourceCalls"].as_array().unwrap().len(), 3);
    assert_eq!(report["sourceCalls"][0]["attempts"], 3);
    assert_eq!(
        report["apiDiagnostics"]["playDeveloperReporting"]["condition"],
        "available"
    );
    assert!(report["findings"].as_array().unwrap().is_empty());
}

#[tokio::test]
async fn quota_exceeded_403_is_retried_like_429() {
    // Some androidpublisher endpoints (e.g. applications.tracks.releases.list)
    // report quota exhaustion as 403 PERMISSION_DENIED with a "quota" message
    // instead of 429 RESOURCE_EXHAUSTED; it must still be retried.
    let quota_exceeded = error(403, "Listing releases quota exceeded.", json!({}));
    let invoker = FakeInvoker::default()
        .with(
            "apps_search",
            vec![
                quota_exceeded.clone(),
                quota_exceeded,
                ok(json!({"apps": [{"packageName": "org.example.app", "name": "apps/123", "displayName": "Example"}]})),
            ],
        )
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "complete");
    assert_eq!(report["sourceCalls"][0]["attempts"], 3);
    assert_eq!(report["sourceCalls"][0]["resultState"], "success");
}

#[tokio::test]
async fn disabled_reporting_keeps_publisher_evidence_and_returns_activation_action() {
    let disabled = error(
        403,
        "SERVICE_DISABLED",
        json!({
            "service": "playdeveloperreporting.googleapis.com",
            "consumer": "projects/123",
            "activationUrl": "https://console.example/enable-reporting",
            "credential": "must-not-leak",
            "purchaseToken": "also-must-not-leak"
        }),
    );
    let invoker = FakeInvoker::default()
        .with(
            "apps_search",
            vec![disabled.clone(), disabled.clone(), disabled],
        )
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "partial");
    assert_eq!(report["apiAvailability"]["androidPublisher"], "available");
    assert_eq!(
        report["apiAvailability"]["playDeveloperReporting"],
        "unavailable"
    );
    assert_eq!(report["sourceCalls"][0]["attempts"], 3);
    assert_eq!(
        report["apiDiagnostics"]["playDeveloperReporting"]["condition"],
        "service_disabled"
    );
    assert!(report["sourceCalls"][0]["startedAt"].is_string());
    assert!(report["sourceCalls"][0]["finishedAt"].is_string());
    assert_eq!(
        report["actions"][0]["activationUrl"],
        "https://console.example/enable-reporting"
    );
    let encoded = report.to_string();
    assert!(!encoded.contains("must-not-leak"));
    assert!(!encoded.contains("also-must-not-leak"));
}

#[tokio::test]
async fn incomplete_pagination_marks_the_source_and_report_partial() {
    let unavailable = error(503, "UNAVAILABLE", json!({}));
    let invoker = FakeInvoker::default()
        .with(
            "apps_search",
            vec![
                ok(json!({"apps": [], "nextPageToken": "page-2"})),
                unavailable.clone(),
                unavailable.clone(),
                unavailable,
            ],
        )
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "partial");
    assert_eq!(report["paginationComplete"], false);
    assert_eq!(report["sourceCalls"][1]["attempts"], 3);
    assert_eq!(report["sourceCalls"][1]["paginationComplete"], false);
}

#[tokio::test]
async fn transient_failure_preserves_successful_release_evidence_as_partial() {
    let unavailable = error(503, "UNAVAILABLE", json!({}));
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [{"type": "PRODUCTION", "displayName": "Production", "servingReleases": []}]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![unavailable.clone(), unavailable.clone(), unavailable],
        );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"]}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "partial");
    assert_eq!(report["tracks"][0]["track"], "production");
    assert_eq!(report["sourceCalls"][1]["attempts"], 3);
    assert_eq!(report["sourceCalls"][1]["resultState"], "error");
}

#[tokio::test]
async fn quality_consumes_all_error_issue_and_anomaly_pages() {
    let invoker = FakeInvoker::default()
        .with("vitals_errors_counts_get", vec![freshness()])
        .with(
            "vitals_errors_counts_query",
            vec![
                ok(json!({"rows": [{"metrics": []}], "nextPageToken": "counts-2"})),
                ok(json!({"rows": [{"metrics": []}, {"metrics": []}]})),
            ],
        )
        .with(
            "vitals_errors_issues_search",
            vec![
                ok(json!({"errorIssues": [{"name": "one"}], "nextPageToken": "issues-2"})),
                ok(json!({"errorIssues": [{"name": "two"}]})),
            ],
        )
        .with(
            "anomalies_list",
            vec![
                ok(json!({"anomalies": [{"name": "one"}], "nextPageToken": "anomalies-2"})),
                ok(json!({"anomalies": [{"name": "two"}, {"name": "three"}]})),
            ],
        );

    let report = run(
        "report_quality_health",
        json!({
            "packageName": "org.example.app",
            "metrics": ["unknown_metric"],
            "includeIssues": true,
            "includeAnomalies": true
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "complete");
    assert_eq!(report["errorEvidence"]["countRows"], 3);
    assert_eq!(report["errorEvidence"]["groupedIssues"], 2);
    assert_eq!(report["errorEvidence"]["countsPaginationComplete"], true);
    assert_eq!(report["errorEvidence"]["issuesPaginationComplete"], true);
    assert_eq!(report["anomalyEvidence"]["count"], 3);
    assert_eq!(report["anomalyEvidence"]["paginationComplete"], true);

    let calls = invoker.calls();
    assert_eq!(calls.len(), 7);
    // counts is a raw-body POST query: the token must land inside `body`, where
    // the executor sends it, not at the top level (which it would drop).
    assert_eq!(calls[2].1["body"]["pageToken"], "counts-2");
    assert!(calls[2].1.get("pageToken").is_none());
    assert_eq!(calls[4].1["pageToken"], "issues-2");
    assert_eq!(calls[6].1["pageToken"], "anomalies-2");
    assert!(calls.iter().all(|(method, _)| registry::allowed(method)));
}

#[tokio::test]
async fn project_status_does_not_surface_raw_review_content() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": []}))],
        )
        .with(
            "reviews_list",
            vec![ok(json!({"reviews": [{
                "authorName": "Private Person",
                "comments": [{"userComment": {"text": "private review text"}}]
            }]}))],
        );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["reviews"]}),
        &invoker,
    )
    .await;
    let encoded = report.to_string();

    assert_eq!(report["reviews"]["returnedReviews"], 1);
    assert!(!encoded.contains("Private Person"));
    assert!(!encoded.contains("private review text"));
}

#[tokio::test]
async fn non_json_source_is_partial_instead_of_counted_as_success() {
    let invoker = FakeInvoker::default()
        .with("apps_search", vec![ToolResult::text("not json")])
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["status"], "partial");
    assert_eq!(report["sourceCalls"][0]["resultState"], "error");
    assert_eq!(report["warnings"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn console_message_is_preserved_as_untrusted_and_never_marked_resolved() {
    let invoker = FakeInvoker::default();
    let message = "Your app must target a newer API level";
    let report = run(
        "report_explain_console_message",
        json!({"packageName": "org.example.app", "message": message, "consoleArea": "Production release"}),
        &invoker,
    )
    .await;

    assert_eq!(report["userInput"]["message"], message);
    assert_eq!(report["userInput"]["untrusted"], true);
    assert_eq!(report["classification"]["category"], "target_api");
    assert_eq!(report["findings"][0]["state"], "inferred");
    assert!(report["findings"][0]["detail"]
        .as_str()
        .unwrap()
        .contains("does not prove"));
}

#[tokio::test]
async fn every_orchestrated_source_call_is_on_the_explicit_allowlist() {
    let invoker = FakeInvoker::default()
        .with("vitals_anrrate_get", vec![freshness()])
        .with("vitals_anrrate_query", vec![ok(json!({"rows": []}))]);
    run(
        "report_quality_health",
        json!({"packageName": "org.example.app", "cohorts": ["OS_PUBLIC"], "metrics": ["anr_rate"], "includeIssues": false, "includeAnomalies": false}),
        &invoker,
    )
    .await;

    assert!(invoker
        .calls()
        .iter()
        .all(|(method, _)| registry::allowed(method)));
}

#[tokio::test]
async fn metric_query_paginates_via_body_token_not_top_level() {
    let invoker = FakeInvoker::default()
        .with("vitals_anrrate_get", vec![freshness()])
        .with(
            "vitals_anrrate_query",
            vec![
                ok(json!({"rows": [zero_row("50", "0.9")], "nextPageToken": "rate-2"})),
                ok(json!({"rows": [zero_row("50", "0.9")]})),
            ],
        );

    let report = run(
        "report_quality_health",
        json!({"packageName": "org.example.app", "cohorts": ["OS_PUBLIC"], "metrics": ["anr_rate"], "includeIssues": false, "includeAnomalies": false}),
        &invoker,
    )
    .await;

    assert_eq!(report["metrics"][0]["returnedObservationCount"], 2);
    let calls = invoker.calls();
    // [0] get, [1] query page 1, [2] query page 2 with the continuation token.
    assert_eq!(calls[2].1["body"]["pageToken"], "rate-2");
    assert!(calls[2].1.get("pageToken").is_none());
}

#[tokio::test]
async fn observed_days_counts_distinct_days_not_dimension_rows() {
    // slow_start_rate has a required startType dimension: 2 days x 3 startTypes = 6
    // rows, but only 2 distinct observed days.
    let mut rows = Vec::new();
    for day in [3, 4] {
        for start_type in ["COLD", "WARM", "HOT"] {
            rows.push(json!({
                "startTime": {"year": 2026, "month": 9, "day": day},
                "dimensions": [{"dimension": "startType", "stringValue": start_type}],
                "metrics": [
                    {"metric": "slowStartRate", "decimalValue": {"value": "0"}},
                    {"metric": "distinctUsers", "decimalValue": {"value": "50"}}
                ]
            }));
        }
    }
    let invoker = FakeInvoker::default()
        .with("vitals_slowstartrate_get", vec![freshness()])
        .with(
            "vitals_slowstartrate_query",
            vec![ok(json!({"rows": rows}))],
        );

    let report = run(
        "report_quality_health",
        json!({"packageName": "org.example.app", "cohorts": ["OS_PUBLIC"], "metrics": ["slow_start_rate"], "includeIssues": false, "includeAnomalies": false}),
        &invoker,
    )
    .await;

    assert_eq!(report["metrics"][0]["observedDays"], 2);
    assert_eq!(report["metrics"][0]["returnedObservationCount"], 6);
    // 2 distinct days is under the 7-day floor, so evidence stays low.
    assert_eq!(report["metrics"][0]["evidenceStrength"], "low");
}

#[tokio::test]
async fn project_status_skips_release_work_when_not_requested() {
    let invoker = FakeInvoker::default();
    let report = run(
        "report_project_status",
        json!({
            "packageName": "org.example.app",
            "include": ["quality"],
            "metrics": ["unknown_metric"],
            "includeIssues": false,
            "includeAnomalies": false
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["releaseState"], "unknown");
    assert!(report["quality"].is_object());
    assert!(invoker
        .calls()
        .iter()
        .all(|(method, _)| method != "apps_fetchReleaseFilterOptions"
            && method != "applications_tracks_releases_list"));
}

#[tokio::test]
async fn reporting_app_absence_is_unknown_when_pagination_fails() {
    // Package missing from page one, then pagination fails: absence is unproven.
    let unavailable = error(503, "UNAVAILABLE", json!({}));
    let invoker = FakeInvoker::default()
        .with(
            "apps_search",
            vec![
                ok(json!({"apps": [{"packageName": "org.other.app"}], "nextPageToken": "page-2"})),
                unavailable.clone(),
                unavailable.clone(),
                unavailable,
            ],
        )
        .with("reviews_list", vec![ok(json!({"reviews": []}))]);

    let report = run(
        "report_capabilities",
        json!({"packageName": "org.example.app", "probe": true}),
        &invoker,
    )
    .await;

    assert_eq!(report["paginationComplete"], false);
    let finding = &report["findings"][0];
    assert_eq!(finding["id"], "reporting-app-access-unknown");
    assert_eq!(finding["state"], "unknown");
    assert_ne!(finding["evidenceStrength"], "high");
    assert_eq!(
        report["applicationAccess"]["playDeveloperReporting"],
        "unknown"
    );
}

#[tokio::test]
async fn explain_skips_release_correlation_for_unknown_console_area() {
    let invoker = FakeInvoker::default();
    let report = run(
        "report_explain_console_message",
        json!({
            "packageName": "org.example.app",
            "message": "Something happened",
            "consoleArea": "Some unrecognized area",
            "versionCode": 42
        }),
        &invoker,
    )
    .await;

    assert!(report["correlatedEvidence"].as_array().unwrap().is_empty());
    // No track could be mapped, so no release list call was issued (would have
    // defaulted to production before the fix).
    assert!(invoker.calls().is_empty());
}

#[tokio::test]
async fn custom_testing_track_uses_source_provided_identifier() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "PRODUCTION", "displayName": "Production", "servingReleases": []},
                {"type": "CLOSED_TESTING", "displayName": "qa-ring", "servingReleases": []}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![
                ok(json!({"releases": [{
                    "releaseName": "1.0.0",
                    "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED",
                    "activeArtifacts": [{"versionCode": 11}]
                }]})),
                ok(json!({"releases": [{
                    "releaseName": "1.1.0",
                    "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED",
                    "activeArtifacts": [{"versionCode": 12}]
                }]})),
            ],
        );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"]}),
        &invoker,
    )
    .await;

    let production = &report["tracks"][0];
    assert_eq!(production["track"], "production");
    assert_eq!(production["trackIdInferred"], false);
    assert_eq!(production["releasesUnresolved"], false);

    let custom = &report["tracks"][1];
    assert_eq!(custom["track"], "qa-ring");
    assert_eq!(custom["trackIdInferred"], false);
    assert_eq!(custom["releasesUnresolved"], false);

    let calls = invoker.calls();
    assert_eq!(
        calls[2].1["parent"],
        "applications/org.example.app/tracks/qa-ring"
    );
    assert_eq!(report["sourceCalls"][2]["id"], "call-qa-ring-releases");
}

#[tokio::test]
async fn duplicate_track_ids_only_issue_one_releases_call() {
    // Two open-testing tracks with no distinguishing displayName both resolve
    // to the default "beta" id; the second must reuse the first's result
    // instead of re-hitting the tight releases-listing quota.
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "OPEN_TESTING", "displayName": "Open A", "servingReleases": []},
                {"type": "OPEN_TESTING", "displayName": "Open B", "servingReleases": []}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![ok(json!({"releases": [{
                "releaseName": "1.0.0",
                "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED",
                "activeArtifacts": [{"versionCode": 11}]
            }]}))],
        );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"]}),
        &invoker,
    )
    .await;

    assert_eq!(report["tracks"][0]["releases"], report["tracks"][1]["releases"]);
    let calls = invoker.calls();
    assert_eq!(
        calls
            .iter()
            .filter(|(method, _)| method == "applications_tracks_releases_list")
            .count(),
        1
    );
}

#[tokio::test]
async fn recovery_lookup_targets_the_detected_serving_version_code() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "PRODUCTION", "displayName": "Production", "servingReleases": [{"displayName": "1.0.0", "versionCodes": ["11"]}]}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![ok(json!({"releases": []}))],
        )
        .with("apprecovery_list", vec![ok(json!({"recoveryActions": []}))]);

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases", "recoveries"]}),
        &invoker,
    )
    .await;

    assert_eq!(report["recoveries"]["count"], 0);
    let recovery_call = invoker
        .calls()
        .into_iter()
        .find(|(method, _)| method == "apprecovery_list")
        .expect("apprecovery_list should have been called");
    assert_eq!(recovery_call.1["versionCode"], "11");
}

#[tokio::test]
async fn recovery_lookup_is_skipped_without_a_known_serving_version() {
    let invoker = FakeInvoker::default().with(
        "apps_fetchReleaseFilterOptions",
        vec![ok(json!({"tracks": []}))],
    );

    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases", "recoveries"]}),
        &invoker,
    )
    .await;

    assert_eq!(report["recoveries"]["count"], 0);
    assert!(invoker
        .calls()
        .iter()
        .all(|(method, _)| method != "apprecovery_list"));
}

#[tokio::test]
async fn error_count_query_sends_a_date_only_end_time_from_freshness() {
    let invoker = FakeInvoker::default()
        .with(
            "vitals_errors_counts_get",
            vec![ok(json!({
                "freshnessInfo": {"freshnesses": [{
                    "aggregationPeriod": "DAILY",
                    "latestEndTime": {
                        "year": 2026, "month": 9, "day": 3, "hours": 7,
                        "timeZone": {"id": "America/Los_Angeles"}
                    }
                }]}
            }))],
        )
        .with(
            "vitals_errors_counts_query",
            vec![ok(json!({"rows": []}))],
        )
        .with("vitals_errors_issues_search", vec![ok(json!({"errorIssues": []}))]);

    run(
        "report_quality_health",
        json!({"packageName": "org.example.app", "metrics": [], "includeAnomalies": false}),
        &invoker,
    )
    .await;

    let calls = invoker.calls();
    let counts_call = &calls
        .iter()
        .find(|(method, _)| method == "vitals_errors_counts_query")
        .unwrap()
        .1;
    let end_time = &counts_call["body"]["timelineSpec"]["endTime"];
    assert_eq!(end_time["day"], 3);
    assert!(end_time.get("hours").is_none());
}

#[tokio::test]
async fn grouped_error_lookup_uses_utc_not_los_angeles() {
    let invoker = FakeInvoker::default()
        .with(
            "vitals_errors_counts_get",
            vec![ok(json!({"freshnessInfo": {"freshnesses": []}}))],
        )
        .with(
            "vitals_errors_counts_query",
            vec![ok(json!({"rows": []}))],
        )
        .with("vitals_errors_issues_search", vec![ok(json!({"errorIssues": []}))]);

    run(
        "report_quality_health",
        json!({"packageName": "org.example.app", "metrics": [], "includeAnomalies": false}),
        &invoker,
    )
    .await;

    let calls = invoker.calls();
    let issues_call = &calls
        .iter()
        .find(|(method, _)| method == "vitals_errors_issues_search")
        .unwrap()
        .1;
    assert_eq!(issues_call["interval.startTime.timeZone.id"], "UTC");
    assert_eq!(issues_call["interval.endTime.timeZone.id"], "UTC");
}

fn rejected_13_with_12_serving() -> FakeInvoker {
    FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "PRODUCTION", "displayName": "Production", "servingReleases": [{"displayName": "1.1", "versionCodes": ["12"]}]}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![ok(json!({"releases": [
                {"releaseName": "1.2", "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_NOT_APPROVED", "activeArtifacts": [{"versionCode": 13}]},
                {"releaseName": "1.1", "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED", "activeArtifacts": [{"versionCode": 12}]}
            ]}))],
        )
}

#[tokio::test]
async fn project_status_reports_requested_release_not_older_serving_one() {
    let invoker = rejected_13_with_12_serving();
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "production", "versionCode": 13}),
        &invoker,
    )
    .await;

    assert_eq!(report["requestedRelease"]["lifecycle"], "rejected");
    assert_eq!(report["requestedRelease"]["servingOnTrack"], false);
    assert_eq!(report["requestedRelease"]["otherServingVersionCodes"], json!(["12"]));
    assert_eq!(report["overallAssessment"], "attention_recommended");
    let summary = report["summary"].as_str().unwrap();
    assert!(summary.contains("13") && summary.contains("rejected"), "{summary}");
    assert_eq!(report["releaseStateScope"], "app_production_latest");
}

#[tokio::test]
async fn project_status_flags_requested_release_missing_from_track() {
    let invoker = rejected_13_with_12_serving();
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "production", "versionCode": 14}),
        &invoker,
    )
    .await;

    assert_eq!(report["requestedRelease"]["found"], false);
    assert_eq!(report["requestedRelease"]["unresolved"].as_array().unwrap().len(), 1);
    assert_eq!(report["findings"][0]["id"], "requested-release-not-live");
}

#[tokio::test]
async fn release_verification_matches_checksum_and_notes_inside_an_edit() {
    let sha = "AB".repeat(32);
    let invoker = rejected_13_with_12_serving()
        .with(
            "edits_bundles_list",
            vec![ok(json!({"bundles": [{"versionCode": 13, "sha256": sha.to_ascii_lowercase()}]}))],
        )
        .with(
            "edits_tracks_get",
            vec![ok(json!({"releases": [{"versionCodes": ["13"], "releaseNotes": [{"language": "en-US", "text": "Fixes. "}]}]}))],
        );
    let report = run(
        "report_release_verification",
        json!({
            "packageName": "org.example.app", "track": "production", "versionCode": 13,
            "expectedChecksum": sha, "releaseNotes": [{"language": "en-US", "text": "Fixes."}], "editId": "e1"
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["lifecycle"], "rejected");
    assert_eq!(report["checks"]["releaseFound"]["result"], "match");
    assert_eq!(report["checks"]["checksum"]["result"], "match");
    assert_eq!(report["checks"]["releaseNotes"]["result"], "match");
    assert_eq!(report["allMatch"], true);
    assert!(invoker.calls().iter().all(|(method, _)| registry::allowed(method)));
}

#[tokio::test]
async fn release_verification_never_claims_unreadable_checks_match() {
    let invoker = rejected_13_with_12_serving();
    let report = run(
        "report_release_verification",
        json!({"packageName": "org.example.app", "track": "production", "versionCode": 13, "expectedChecksum": "a".repeat(64)}),
        &invoker,
    )
    .await;

    assert_eq!(report["checks"]["checksum"]["result"], "unverifiable");
    assert_eq!(report["checks"]["releaseNotes"]["result"], "not_requested");
    assert_eq!(report["allMatch"], false);
    assert_eq!(invoker.calls().len(), 1);
}

#[tokio::test]
async fn release_verification_reports_checksum_mismatch_and_missing_version() {
    let invoker = rejected_13_with_12_serving().with(
        "edits_bundles_list",
        vec![ok(json!({"bundles": [{"versionCode": 13, "sha256": "b".repeat(64)}]}))],
    );
    let report = run(
        "report_release_verification",
        json!({"packageName": "org.example.app", "track": "production", "versionCode": 13, "expectedChecksum": "a".repeat(64), "editId": "e1"}),
        &invoker,
    )
    .await;
    assert_eq!(report["checks"]["checksum"]["result"], "mismatch");
    assert_eq!(report["allMatch"], false);

    let report = run(
        "report_release_verification",
        json!({"packageName": "org.example.app", "track": "production", "versionCode": 99}),
        &rejected_13_with_12_serving(),
    )
    .await;
    assert_eq!(report["checks"]["releaseFound"]["result"], "mismatch");
}

#[tokio::test]
async fn foreground_service_rejection_gets_safe_concrete_steps() {
    let message = "Your app uses the FOREGROUND_SERVICE_DATA_SYNC permission but has not completed the foreground service permissions declaration.";
    let report = run(
        "report_explain_console_message",
        json!({"packageName": "org.example.app", "message": message}),
        &FakeInvoker::default(),
    )
    .await;

    assert_eq!(report["classification"]["category"], "foreground_service");
    assert_eq!(report["findings"][0]["exactError"], message);
    let actions = report["actions"].as_array().unwrap();
    assert!(actions[0]["title"].as_str().unwrap().contains("merged manifest"));
    assert!(actions.iter().any(|action| action["requiresConsole"] == true));
    assert!(actions
        .iter()
        .all(|action| !action["title"].as_str().unwrap().starts_with("Remove")));
    assert!(report["coverageGaps"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gap| gap["area"] == "foreground_service_declaration"));
}

#[tokio::test]
async fn project_status_queries_a_requested_track_the_reporting_api_omits() {
    let invoker = rejected_13_with_12_serving().with(
        "applications_tracks_releases_list",
        vec![
            ok(json!({"releases": []})),
            ok(json!({"releases": [{"releaseName": "qa", "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_PUBLISHED", "activeArtifacts": [{"versionCode": 13}]}]})),
        ],
    );
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "qa-team", "versionCode": 13}),
        &invoker,
    )
    .await;

    assert_eq!(report["requestedRelease"]["found"], true);
    assert_eq!(report["requestedRelease"]["lifecycle"], "published");
    assert!(report["requestedRelease"]["servingOnTrack"].is_null());
    assert!(invoker.calls().iter().any(|(_, args)| args["parent"]
        == "applications/org.example.app/tracks/qa-team"));
}

#[tokio::test]
async fn project_status_does_not_call_a_failed_lookup_missing() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [
                {"type": "PRODUCTION", "displayName": "Production", "servingReleases": [{"versionCodes": ["12"]}]}
            ]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![error(404, "not found", json!({}))],
        );
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "production", "versionCode": 13}),
        &invoker,
    )
    .await;

    assert!(report["requestedRelease"]["found"].is_null());
    let unresolved = report["requestedRelease"]["unresolved"].to_string();
    assert!(unresolved.contains("could not be checked"), "{unresolved}");
    assert!(!unresolved.contains("No active release"), "{unresolved}");
    assert_eq!(report["findings"][0]["title"], "Requested release could not be checked");
    assert!(report["summary"].as_str().unwrap().contains("could not be checked"));
}

#[tokio::test]
async fn project_status_warns_on_unpublished_requested_release() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": [{"type": "PRODUCTION", "displayName": "Production", "servingReleases": []}]}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![ok(json!({"releases": [{"releaseName": "1.2", "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_DRAFT", "activeArtifacts": [{"versionCode": 13}]}]}))],
        );
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "production", "versionCode": 13}),
        &invoker,
    )
    .await;

    assert_eq!(report["requestedRelease"]["lifecycle"], "draft");
    assert_eq!(report["findings"][0]["id"], "requested-release-not-live");
    assert_eq!(report["overallAssessment"], "attention_recommended");
}

#[tokio::test]
async fn project_status_flags_version_code_without_track() {
    let invoker = rejected_13_with_12_serving();
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "versionCode": 13}),
        &invoker,
    )
    .await;

    assert!(report["requestedRelease"]["found"].is_null());
    assert!(report["summary"].as_str().unwrap().contains("Both track and versionCode"));
    assert_eq!(report["overallAssessment"], "attention_recommended");
}

#[tokio::test]
async fn foreground_service_wins_over_broader_categories() {
    let report = run(
        "report_explain_console_message",
        json!({
            "packageName": "org.example.app",
            "message": "Your app bundle uses foreground service permissions without a declaration.",
            "consoleArea": "App content"
        }),
        &FakeInvoker::default(),
    )
    .await;

    assert_eq!(report["classification"]["category"], "foreground_service");
}

#[tokio::test]
async fn release_verification_marks_missing_hash_unverifiable_and_labels_edit_source() {
    let invoker = rejected_13_with_12_serving()
        .with(
            "edits_bundles_list",
            vec![ok(json!({"bundles": [{"versionCode": 13}]}))],
        )
        .with(
            "edits_tracks_get",
            vec![ok(json!({"releases": [{"versionCodes": ["13"], "releaseNotes": [{"language": "en-US", "text": "Fixes."}]}]}))],
        );
    let report = run(
        "report_release_verification",
        json!({
            "packageName": "org.example.app", "track": "production", "versionCode": 13,
            "expectedChecksum": "a".repeat(64), "releaseNotes": [{"language": "en-US", "text": "Fixes."}], "editId": "e1"
        }),
        &invoker,
    )
    .await;

    assert_eq!(report["checks"]["checksum"]["result"], "unverifiable");
    assert!(report["checks"]["releaseNotes"]["source"]
        .as_str()
        .unwrap()
        .contains("uncommitted"));
    assert_eq!(report["allMatch"], false);
}

#[tokio::test]
async fn directly_looked_up_production_does_not_claim_it_is_not_serving() {
    let invoker = FakeInvoker::default()
        .with(
            "apps_fetchReleaseFilterOptions",
            vec![ok(json!({"tracks": []}))],
        )
        .with(
            "applications_tracks_releases_list",
            vec![ok(json!({"releases": [{"releaseName": "1.2", "releaseLifecycleState": "RELEASE_LIFECYCLE_STATE_IN_REVIEW", "activeArtifacts": [{"versionCode": 13}]}]}))],
        );
    let report = run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "production", "versionCode": 13}),
        &invoker,
    )
    .await;

    assert_eq!(report["requestedRelease"]["lifecycle"], "under_review");
    assert!(report["requestedRelease"]["servingOnTrack"].is_null());
    assert!(report["publicServingVersionCodes"].is_null());
    assert!(report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .all(|finding| finding["id"] != "production-release-in-review"));
}

#[tokio::test]
async fn lone_track_does_not_spend_a_direct_releases_call() {
    let invoker = rejected_13_with_12_serving();
    run(
        "report_project_status",
        json!({"packageName": "org.example.app", "include": ["releases"], "track": "qa-team"}),
        &invoker,
    )
    .await;

    assert!(invoker
        .calls()
        .iter()
        .all(|(_, args)| args["parent"] != "applications/org.example.app/tracks/qa-team"));
}
