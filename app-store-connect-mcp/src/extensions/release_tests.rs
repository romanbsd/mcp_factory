//! Workflow contracts through the real HTTP executor and internal catalog.
use mcp_factory_core::{McpProxyServer, ProxyConfig};
use serde_json::{json, Value};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn server(mock: &MockServer) -> McpProxyServer {
    let cfg = ProxyConfig {
        base_url: mock.uri(),
        profiles: Some(vec!["workflow".into()]),
        ..Default::default()
    };
    super::test_support::server(cfg)
}
fn args() -> Value {
    json!({"bundleId":"com.example.release","version":"1.0"})
}
async fn get(mock: &MockServer, endpoint: &str, data: Value) {
    Mock::given(method("GET"))
        .and(path(endpoint))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":data})))
        .mount(mock)
        .await;
}
async fn resolve(mock: &MockServer, state: &str) {
    Mock::given(method("GET")).and(path("/v1/apps"))
        .and(query_param("filter[bundleId]", "com.example.release"))
        .and(|request: &wiremock::Request| { request.url.query_pairs().filter(|(k,_)| k == "fields[apps]").map(|(_,v)|v.into_owned()).collect::<Vec<_>>() == ["name,bundleId,sku,primaryLocale"] })
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"app","type":"apps","attributes":{"bundleId":"com.example.release","name":"Example","sku":"example","primaryLocale":"en-US","noise":"x".repeat(8000)}}]})))
        .mount(mock).await;
    Mock::given(method("GET")).and(path("/v1/apps/app/appStoreVersions"))
        .and(query_param("filter[versionString]", "1.0"))
        .and(query_param("filter[platform]", "IOS"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"version","type":"appStoreVersions","attributes":{"versionString":"1.0","platform":"IOS","appStoreState":state,"releaseType":"MANUAL","noise":"x".repeat(8000)}}]})))
        .mount(mock).await;
}
async fn build(mock: &MockServer, present: bool) {
    get(mock,"/v1/appStoreVersions/version/build", if present {json!({"id":"build","type":"builds","attributes":{"version":"42","processingState":"VALID","expired":false,"usesNonExemptEncryption":false,"expirationDate":"2027-01-01T00:00:00Z"}})} else {Value::Null}).await;
}
async fn review(mock: &MockServer, state: &str, versions: &[&str]) {
    Mock::given(method("GET")).and(path("/v1/reviewSubmissions"))
        .and(query_param("filter[app]", "app")).and(query_param("filter[platform]", "IOS"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"submission","type":"reviewSubmissions","attributes":{"state":state,"platform":"IOS"}}]})))
        .mount(mock).await;
    get(mock,"/v1/reviewSubmissions/submission/items",json!(versions.iter().enumerate().map(|(n,id)|json!({"id":format!("item-{n}"),"type":"reviewSubmissionItems","relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":id}}}})).collect::<Vec<_>>())).await;
}
async fn preflight(mock: &MockServer, complete: bool) {
    build(mock, complete).await;
    get(mock,"/v1/appStoreVersions/version/appStoreVersionLocalizations",if complete {json!([{"id":"locale","type":"appStoreVersionLocalizations","attributes":{"locale":"en-US","description":"A useful app","supportUrl":"https://example.com/support"}}])}else{json!([])}).await;
    if complete {
        get(mock,"/v1/appStoreVersionLocalizations/locale/appScreenshotSets",json!([{"id":"set","type":"appScreenshotSets","attributes":{"screenshotDisplayType":"APP_IPHONE_67"}}])).await;
        get(mock,"/v1/appScreenshotSets/set/appScreenshots",json!([{"id":"shot","type":"appScreenshots","attributes":{"assetDeliveryState":{"state":"COMPLETE"}}}])).await;
        get(mock,"/v1/appStoreVersions/version/appStoreReviewDetail",json!({"id":"detail","type":"appStoreReviewDetails","attributes":{"contactFirstName":"First","contactLastName":"Last","contactPhone":"+123456789","contactEmail":"review@example.com","demoAccountRequired":false}})).await;
    } else {
        Mock::given(method("GET")).and(path("/v1/appStoreVersions/version/appStoreReviewDetail"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"errors":[{"status":"404","code":"NOT_FOUND","title":"Missing relationship"}]}))).mount(mock).await;
    }
    get(mock,"/v1/apps/app/appInfos",json!([{"id":"info","type":"appInfos","attributes":{"appStoreState":"PREPARE_FOR_SUBMISSION"}}])).await;
    get(mock,"/v1/appInfos/info/ageRatingDeclaration",json!({"id":"age","type":"ageRatingDeclarations","attributes":{"violenceCartoonOrFantasy":"NONE"}})).await;
}
async fn call(server: &McpProxyServer, name: &str, args: Value, max_bytes: usize) -> Value {
    let text = server.invoke_tool(name, args).await.unwrap();
    assert!(
        text.len() <= max_bytes,
        "{name} returned {} bytes",
        text.len()
    );
    serde_json::from_str(&text).unwrap()
}
async fn assert_no_writes(mock: &MockServer) {
    for request in mock.received_requests().await.unwrap() {
        assert_eq!(
            request.method.as_str(),
            "GET",
            "unexpected mutation: {} {}",
            request.method,
            request.url
        );
    }
}

#[tokio::test]
async fn app_and_version_discovery_are_compact_and_exactly_scoped() {
    let mock = MockServer::start().await;
    resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
    build(&mock, true).await;
    let srv = server(&mock);
    let app = call(
        &srv,
        "app_get",
        json!({"bundleId":"com.example.release"}),
        2000,
    )
    .await;
    assert_eq!(
        app,
        json!({"id":"app","bundleId":"com.example.release","name":"Example","sku":"example","primaryLocale":"en-US"})
    );
    let version = call(&srv, "version_get", args(), 3000).await;
    assert_eq!(
        version["version"]["appStoreState"],
        "PREPARE_FOR_SUBMISSION"
    );
    assert_eq!(version["build"]["version"], "42");
    assert!(version["version"].get("noise").is_none());
    assert_no_writes(&mock).await;
}
#[tokio::test]
async fn status_combines_attached_build_review_and_phased_release() {
    let mock = MockServer::start().await;
    resolve(&mock, "WAITING_FOR_REVIEW").await;
    build(&mock, true).await;
    review(&mock, "WAITING_FOR_REVIEW", &["version"]).await;
    get(&mock,"/v1/appStoreVersions/version/appStoreVersionPhasedRelease",json!({"id":"phased","type":"appStoreVersionPhasedReleases","attributes":{"phasedReleaseState":"INACTIVE","currentDayNumber":0,"totalPauseDuration":0}})).await;
    let report = call(&server(&mock), "release_status", args(), 5000).await;
    assert_eq!(report["build"]["id"], "build");
    assert_eq!(
        report["reviewSubmissions"][0]["state"],
        "WAITING_FOR_REVIEW"
    );
    assert_eq!(report["phasedRelease"]["phasedReleaseState"], "INACTIVE");
    assert_eq!(report["blockingIssues"], json!([]));
    assert_no_writes(&mock).await;
}
#[tokio::test]
async fn prepare_reports_known_blockers_and_unknown_apple_checks() {
    let mock = MockServer::start().await;
    resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
    preflight(&mock, false).await;
    let report = call(&server(&mock), "release_prepare", args(), 10000).await;
    assert_eq!(report["readyForSubmission"], false);
    let issues = report["blockingIssues"].as_array().unwrap();
    assert!(issues.contains(&json!("No build attached")));
    assert!(issues.contains(&json!("Primary localization en-US is missing or ambiguous")));
    assert!(issues.contains(&json!("App Review details are missing")));
    assert!(report["unknownChecks"]
        .as_array()
        .unwrap()
        .contains(&json!("Apple submission validation")));
    assert_no_writes(&mock).await;
}
#[tokio::test]
async fn submitted_matching_review_is_a_noop_without_preflight_or_mutations() {
    let mock = MockServer::start().await;
    resolve(&mock, "WAITING_FOR_REVIEW").await;
    review(&mock, "WAITING_FOR_REVIEW", &["version"]).await;
    let report = call(&server(&mock), "review_submit", args(), 5000).await;
    assert_eq!(report["changes"], json!([]));
    assert_eq!(report["unchanged"], 1);
    assert_no_writes(&mock).await;
    assert!(!mock
        .received_requests()
        .await
        .unwrap()
        .iter()
        .any(|r| r.url.path().ends_with("/build")));
}
#[tokio::test]
async fn matching_draft_runs_preflight_and_only_patches_submitted() {
    let mock = MockServer::start().await;
    resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
    review(&mock, "READY_FOR_REVIEW", &["version"]).await;
    preflight(&mock, true).await;
    Mock::given(method("PATCH")).and(path("/v1/reviewSubmissions/submission"))
        .and(body_json(json!({"data":{"type":"reviewSubmissions","id":"submission","attributes":{"submitted":true}}})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"id":"submission","type":"reviewSubmissions","attributes":{"state":"WAITING_FOR_REVIEW"}}})))
        .expect(1).mount(&mock).await;
    let report = call(&server(&mock), "review_submit", args(), 10000).await;
    assert_eq!(
        report["changes"],
        json!([{"action":"submit","resource":"reviewSubmission"}])
    );
    let requests = mock.received_requests().await.unwrap();
    assert_eq!(
        requests
            .iter()
            .filter(|r| r.method.as_str() != "GET")
            .count(),
        1
    );
    assert!(requests
        .iter()
        .any(|r| r.url.path() == "/v1/appInfos/info/ageRatingDeclaration"));
    assert!(requests
        .iter()
        .any(|r| r.url.path() == "/v1/appScreenshotSets/set/appScreenshots"));
    let patch = requests
        .iter()
        .position(|r| r.method.as_str() == "PATCH")
        .unwrap();
    assert_eq!(patch, requests.len() - 1);
}
#[tokio::test]
async fn unknown_or_mismatched_drafts_fail_before_any_write() {
    for (state, versions) in [
        ("UNKNOWN", vec!["version"]),
        ("READY_FOR_REVIEW", vec!["different"]),
        ("UNKNOWN", vec!["different"]),
        ("READY_FOR_REVIEW", vec!["version", "different"]),
    ] {
        let mock = MockServer::start().await;
        resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
        review(&mock, state, &versions).await;
        preflight(&mock, true).await;
        let error = server(&mock)
            .invoke_tool("review_submit", args())
            .await
            .unwrap_err()
            .to_string();
        assert!(error.contains("reconcile"), "unexpected failure: {error}");
        assert_no_writes(&mock).await;
    }
}
#[tokio::test]
async fn manual_release_dry_run_and_exact_post_are_separate_from_review() {
    let mock = MockServer::start().await;
    resolve(&mock, "PENDING_DEVELOPER_RELEASE").await;
    let srv = server(&mock);
    let mut request = args();
    request["dryRun"] = json!(true);
    let report = call(&srv, "release", request, 5000).await;
    assert_eq!(report["dryRun"], true);
    assert_no_writes(&mock).await;
    Mock::given(method("POST")).and(path("/v1/appStoreVersionReleaseRequests"))
        .and(body_json(json!({"data":{"type":"appStoreVersionReleaseRequests","relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":"version"}}}}})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"data":{"id":"release-request","type":"appStoreVersionReleaseRequests"}})))
        .expect(1).mount(&mock).await;
    let report = call(&srv, "release", args(), 5000).await;
    assert_eq!(report["releaseRequestId"], "release-request");
    assert_eq!(
        mock.received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|r| r.method.as_str() != "GET")
            .count(),
        1
    );
}
#[tokio::test]
async fn manual_release_refuses_unapproved_state() {
    let mock = MockServer::start().await;
    resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
    let error = server(&mock)
        .invoke_tool("release", args())
        .await
        .unwrap_err()
        .to_string();
    assert!(error.contains("PENDING_DEVELOPER_RELEASE"));
    assert_no_writes(&mock).await;
}

#[tokio::test]
async fn fresh_submission_and_completed_history_use_create_item_submit_choreography() {
    for archived in [false, true] {
        let mock = MockServer::start().await;
        resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
        if archived {
            review(&mock, "COMPLETE", &["version"]).await;
        } else {
            get(&mock, "/v1/reviewSubmissions", json!([])).await;
        }
        preflight(&mock, true).await;
        Mock::given(method("POST")).and(path("/v1/reviewSubmissions"))
            .and(body_json(json!({"data":{"type":"reviewSubmissions","attributes":{"platform":"IOS"},"relationships":{"app":{"data":{"type":"apps","id":"app"}}}}})))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({"data":{"type":"reviewSubmissions","id":"new-submission","attributes":{"state":"READY_FOR_REVIEW"}}})))
            .expect(1).mount(&mock).await;
        Mock::given(method("POST")).and(path("/v1/reviewSubmissionItems"))
            .and(body_json(json!({"data":{"type":"reviewSubmissionItems","relationships":{"reviewSubmission":{"data":{"type":"reviewSubmissions","id":"new-submission"}},"appStoreVersion":{"data":{"type":"appStoreVersions","id":"version"}}}}})))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({"data":{"type":"reviewSubmissionItems","id":"new-item"}})))
            .expect(1).mount(&mock).await;
        Mock::given(method("PATCH")).and(path("/v1/reviewSubmissions/new-submission"))
            .and(body_json(json!({"data":{"type":"reviewSubmissions","id":"new-submission","attributes":{"submitted":true}}})))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"type":"reviewSubmissions","id":"new-submission","attributes":{"state":"WAITING_FOR_REVIEW"}}})))
            .expect(1).mount(&mock).await;
        let report = call(&server(&mock), "review_submit", args(), 10000).await;
        assert_eq!(report["submission"]["id"], "new-submission");
        assert_eq!(
            report["changes"],
            json!([
                {"action":"create","resource":"reviewSubmission"},
                {"action":"create","resource":"reviewSubmissionItem","versionId":"version"},
                {"action":"submit","resource":"reviewSubmission"}
            ])
        );
        let requests = mock.received_requests().await.unwrap();
        let mutations: Vec<_> = requests
            .iter()
            .filter(|r| r.method.as_str() != "GET")
            .map(|r| (r.method.as_str(), r.url.path()))
            .collect();
        assert_eq!(
            mutations,
            vec![
                ("POST", "/v1/reviewSubmissions"),
                ("POST", "/v1/reviewSubmissionItems"),
                ("PATCH", "/v1/reviewSubmissions/new-submission")
            ]
        );
        let first_write = requests
            .iter()
            .position(|r| r.method.as_str() != "GET")
            .unwrap();
        assert!(requests[..first_write]
            .iter()
            .any(|r| r.url.path() == "/v1/appInfos/info/ageRatingDeclaration"));
    }
}

#[tokio::test]
async fn completed_history_does_not_hide_existing_usable_draft_for_editable_version() {
    let mock = MockServer::start().await;
    resolve(&mock, "PREPARE_FOR_SUBMISSION").await;
    get(&mock, "/v1/reviewSubmissions", json!([
        {"id":"archived","type":"reviewSubmissions","attributes":{"state":"COMPLETE","platform":"IOS"}},
        {"id":"draft","type":"reviewSubmissions","attributes":{"state":"READY_FOR_REVIEW","platform":"IOS"}}
    ])).await;
    for id in ["archived", "draft"] {
        get(&mock, &format!("/v1/reviewSubmissions/{id}/items"), json!([{
            "id":format!("{id}-item"),"type":"reviewSubmissionItems",
            "relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":"version"}}}
        }])).await;
    }
    preflight(&mock, true).await;
    Mock::given(method("PATCH")).and(path("/v1/reviewSubmissions/draft"))
        .and(body_json(json!({"data":{"type":"reviewSubmissions","id":"draft","attributes":{"submitted":true}}})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":{"type":"reviewSubmissions","id":"draft","attributes":{"state":"WAITING_FOR_REVIEW"}}})))
        .expect(1).mount(&mock).await;
    let report = call(&server(&mock), "review_submit", args(), 10000).await;
    assert_eq!(report["submission"]["id"], "draft");
    assert_eq!(
        report["changes"],
        json!([{"action":"submit","resource":"reviewSubmission"}])
    );
    let requests = mock.received_requests().await.unwrap();
    let mutations: Vec<_> = requests
        .iter()
        .filter(|r| r.method.as_str() != "GET")
        .map(|r| (r.method.as_str(), r.url.path()))
        .collect();
    assert_eq!(mutations, vec![("PATCH", "/v1/reviewSubmissions/draft")]);
}

#[tokio::test]
async fn already_released_noop_preserves_dry_run_and_never_requests_release() {
    let mock = MockServer::start().await;
    resolve(&mock, "READY_FOR_DISTRIBUTION").await;
    let srv = server(&mock);
    for dry in [true, false] {
        let mut input = args();
        input["dryRun"] = json!(dry);
        let report = call(&srv, "release", input, 10000).await;
        assert_eq!(report["state"], "READY_FOR_DISTRIBUTION");
        assert_eq!(report["unchanged"], 1);
        assert_eq!(report["changes"], json!([]));
        assert_eq!(report["dryRun"], dry);
    }
    assert!(mock
        .received_requests()
        .await
        .unwrap()
        .iter()
        .all(|request| request.method.as_str() == "GET"));
}
