//! Generation, hint and mock-API integration tests for the generated App Store
//! Connect server. No real credentials are used: auth is either off or an
//! ES256 key generated per test run.

use std::collections::{BTreeSet, HashMap};
use std::sync::Arc;
use std::time::Duration;

use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;
use mcp_factory_core::{
    AuthConfig, ExecutionKind, McpProxyServer, ProxyConfig, ReadOnlyToolInvoker, ToolSpec,
};
use md5::{Digest, Md5};
use serde_json::{json, Value};
use wiremock::matchers::{body_bytes, body_partial_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

use super::asset_upload::{enabled_kinds, AssetUpload, KINDS};

fn generated() -> Vec<ToolSpec> {
    crate::tools::build_tools()
}

fn by_name() -> HashMap<String, ToolSpec> {
    generated()
        .into_iter()
        .map(|tool| (tool.name.clone(), tool))
        .collect()
}

fn profiles() -> HashMap<&'static str, &'static [&'static str]> {
    crate::tools::build_tool_profiles().into_iter().collect()
}

fn config(base_url: &str, profiles: &[&str]) -> ProxyConfig {
    ProxyConfig {
        base_url: base_url.to_string(),
        compact_jsonapi: Some(true),
        profiles: Some(profiles.iter().map(|name| name.to_string()).collect()),
        ..ProxyConfig::default()
    }
}

fn server(config: ProxyConfig) -> McpProxyServer {
    let custom = super::build_custom_tools(&config);
    McpProxyServer::builder(config)
        .tool_profiles(&crate::tools::build_tool_profiles())
        .unwrap()
        .tools(&generated())
        .unwrap()
        .custom_tools(&custom)
        .unwrap()
        .build()
        .unwrap()
}

fn rest_method(tool: &ToolSpec) -> &str {
    match &tool.execution {
        ExecutionKind::Rest(operation) => &operation.method,
        ExecutionKind::GraphQL(_) => "GRAPHQL",
    }
}

// --- generation -----------------------------------------------------------

#[test]
fn every_generated_tool_has_a_profile_and_defaults_are_known() {
    let profiles = profiles();
    for tool in generated() {
        assert!(
            profiles
                .get(tool.name.as_str())
                .is_some_and(|names| !names.is_empty()),
            "{} has no profile",
            tool.name
        );
    }
    let known: BTreeSet<&str> = profiles
        .values()
        .flat_map(|names| names.iter().copied())
        .collect();
    for profile in crate::tools::default_profiles() {
        assert!(known.contains(profile.as_str()), "{profile}");
    }
    assert_eq!(
        crate::tools::default_profiles(),
        ["core", "metadata", "assets", "testflight"]
    );
}

#[test]
fn representative_operations_per_profile() {
    let profiles = profiles();
    let expect = [
        ("core", "reviewSubmissions_updateInstance"),
        ("core", "appStoreVersions_build_updateToOneRelationship"),
        ("metadata", "appStoreVersionLocalizations_getInstance"),
        ("metadata", "appInfoLocalizations_updateInstance"),
        (
            "assets",
            "appScreenshotSets_appScreenshots_replaceToManyRelationship",
        ),
        ("assets", "asset_upload"),
        (
            "testflight",
            "betaGroups_betaTesters_createToManyRelationship",
        ),
        ("testflight", "betaAppReviewDetails_updateInstance"),
        ("monetization", "subscriptions_createInstance"),
        ("monetization", "asset_upload"),
        ("pricing", "territories_getCollection"),
        ("signing", "certificates_deleteInstance"),
        ("reviews", "customerReviewResponses_createInstance"),
        ("reports", "analytics_segment_download"),
    ];
    for (profile, tool) in expect {
        assert!(
            profiles
                .get(tool)
                .is_some_and(|names| names.contains(&profile)),
            "{tool} not in {profile}"
        );
    }
}

#[test]
fn disabled_profiles_are_absent_at_runtime() {
    let server = server(config("http://127.0.0.1:9", &["core"]));
    let names: BTreeSet<String> = server.tool_names().into_iter().collect();
    assert!(names.contains("apps_getCollection"));
    assert!(!names.contains("certificates_deleteInstance"));
    assert!(!names.contains("asset_upload"));
    assert!(!names.contains("subscriptions_createInstance"));

    let all = server_names(&["all"]);
    assert_eq!(all.len(), generated().len() + 2);
}

fn server_names(profiles: &[&str]) -> BTreeSet<String> {
    server(config("http://127.0.0.1:9", profiles))
        .tool_names()
        .into_iter()
        .collect()
}

#[test]
fn private_api_and_app_creation_are_not_exposed() {
    let tools = by_name();
    assert!(!tools.contains_key("apps_createInstance"));
    assert!(tools.keys().all(|name| !name.starts_with("buildUpload")));
}

// --- hints ---------------------------------------------------------------

#[test]
fn hints_follow_http_semantics() {
    for tool in generated() {
        let hints = &tool.hints;
        match rest_method(&tool) {
            "GET" => {
                assert_eq!(hints.read_only, Some(true), "{}", tool.name);
                assert_eq!(hints.destructive, Some(false), "{}", tool.name);
            }
            "DELETE" => {
                assert_eq!(hints.destructive, Some(true), "{}", tool.name);
                assert_eq!(hints.read_only, Some(false), "{}", tool.name);
            }
            "POST" => {
                assert_eq!(hints.idempotent, Some(false), "{} is a create", tool.name);
                assert_eq!(hints.read_only, Some(false), "{}", tool.name);
            }
            "PATCH" => assert_eq!(hints.read_only, Some(false), "{}", tool.name),
            other => panic!("unexpected method {other} for {}", tool.name),
        }
        assert!(
            !tool
                .description
                .contains("Generated from OpenAPI operation"),
            "{} still has the generic description",
            tool.name
        );
    }
    let tools = by_name();
    for destructive in [
        "certificates_deleteInstance",
        "profiles_deleteInstance",
        "betaTesters_deleteInstance",
        "appScreenshots_deleteInstance",
        "appPreviews_deleteInstance",
        "betaGroups_betaTesters_deleteToManyRelationship",
    ] {
        assert_eq!(
            tools[destructive].hints.destructive,
            Some(true),
            "{destructive}"
        );
    }
}

// --- schemas -------------------------------------------------------------

fn properties(tool: &str) -> Value {
    by_name()[tool].input_schema["properties"].clone()
}

#[test]
fn schemas_keep_filters_pagination_enums_and_relationship_payloads() {
    let apps = properties("apps_getCollection");
    assert!(apps.get("filter[bundleId]").is_some());
    assert!(apps.get("limit").is_some());
    assert_eq!(apps["cursor"]["type"], "string");
    // Sparse fieldsets keep the parameter but drop the long enum.
    assert!(apps["fields[apps]"]["items"].get("enum").is_none());

    let version = properties("appStoreVersions_updateInstance");
    assert_eq!(
        version["data"]["properties"]["attributes"]["properties"]["releaseType"]["enum"],
        json!(["MANUAL", "AFTER_APPROVAL", "SCHEDULED"])
    );
    assert_eq!(
        by_name()["appStoreVersions_updateInstance"].input_schema["required"],
        json!(["id", "data"])
    );

    let reserve = properties("appScreenshots_createInstance");
    let data = &reserve["data"];
    assert_eq!(
        data["properties"]["attributes"]["required"],
        json!(["fileName", "fileSize"])
    );
    assert_eq!(
        data["properties"]["relationships"]["properties"]["appScreenshotSet"]["properties"]["data"]
            ["properties"]["type"]["enum"],
        json!(["appScreenshotSets"])
    );

    let link = properties("betaGroups_betaTesters_createToManyRelationship");
    assert_eq!(link["data"]["type"], "array");

    // Creates are never given the cursor parameter.
    assert!(properties("appStoreVersions_createInstance")
        .get("cursor")
        .is_none());
}

// --- integration: core flows ---------------------------------------------

/// Sets environment variables for one test and restores their previous
/// values (or removes them) on drop, even when the test panics.
struct EnvGuard(Vec<(&'static str, Option<std::ffi::OsString>)>);

impl EnvGuard {
    fn set(vars: &[(&'static str, &str)]) -> Self {
        let previous = vars
            .iter()
            .map(|(key, _)| (*key, std::env::var_os(key)))
            .collect();
        for (key, value) in vars {
            std::env::set_var(key, value);
        }
        Self(previous)
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in &self.0 {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

#[tokio::test]
async fn paginated_app_listing_follows_the_cursor() {
    let api = MockServer::start().await;
    let next = format!("{}/v1/apps?cursor=Mg.page2&limit=1", api.uri());
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .and(query_param("cursor", "Mg.page2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"type": "apps", "id": "2", "attributes": {"name": "Second"},
                      "links": {"self": "s"}}],
            "links": {"self": "s"},
            "meta": {"paging": {"total": 2, "limit": 1}}
        })))
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"type": "apps", "id": "1", "attributes": {"name": "First"},
                      "relationships": {"builds": {"links": {"related": "r"}}},
                      "links": {"self": "s"}}],
            "links": {"self": "s", "next": next},
            "meta": {"paging": {"total": 2, "limit": 1}}
        })))
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["core"]));

    let first = server
        .invoke_read_only("apps_getCollection", json!({"limit": 1}))
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(first["data"][0]["id"], "1");
    assert!(first["data"][0].get("relationships").is_none(), "compacted");
    assert_eq!(first["meta"]["paging"]["total"], 2);
    let next_link = first["links"]["next"]
        .as_str()
        .expect("page one is not complete");
    let cursor = reqwest::Url::parse(next_link)
        .unwrap()
        .query_pairs()
        .find(|(key, _)| key == "cursor")
        .unwrap()
        .1
        .into_owned();

    let second = server
        .invoke_read_only("apps_getCollection", json!({"limit": 1, "cursor": cursor}))
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(second["data"][0]["id"], "2");
    assert!(second["links"].get("next").is_none());
}

#[tokio::test]
async fn reads_then_updates_a_version_localization() {
    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/appStoreVersionLocalizations/loc-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {"type": "appStoreVersionLocalizations", "id": "loc-1",
                     "attributes": {"locale": "en-US", "whatsNew": "Old", "keywords": "a,b"}}
        })))
        .mount(&api)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v1/appStoreVersionLocalizations/loc-1"))
        .and(body_partial_json(json!({"data": {
            "type": "appStoreVersionLocalizations", "id": "loc-1",
            "attributes": {"whatsNew": "Bug fixes"}
        }})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {"type": "appStoreVersionLocalizations", "id": "loc-1",
                     "attributes": {"locale": "en-US", "whatsNew": "Bug fixes"}}
        })))
        .expect(1)
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["metadata"]));

    let current = server
        .invoke_read_only(
            "appStoreVersionLocalizations_getInstance",
            json!({"id": "loc-1"}),
        )
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(current["data"]["attributes"]["keywords"], "a,b");

    let updated = server
        .invoke_tool(
            "appStoreVersionLocalizations_updateInstance",
            json!({"id": "loc-1", "data": {"type": "appStoreVersionLocalizations", "id": "loc-1",
                   "attributes": {"whatsNew": "Bug fixes"}}}),
        )
        .await
        .unwrap();
    assert!(updated.contains("Bug fixes"));
}

#[tokio::test]
async fn beta_group_membership_add_and_list() {
    let api = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/betaGroups/g1/relationships/betaTesters"))
        .and(body_partial_json(
            json!({"data": [{"type": "betaTesters", "id": "t1"}]}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/betaGroups/g1/betaTesters"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"type": "betaTesters", "id": "t1", "attributes": {"email": "qa@example.com"}}],
            "links": {"self": "s"}
        })))
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["testflight"]));

    let added = server
        .invoke_tool(
            "betaGroups_betaTesters_createToManyRelationship",
            json!({"id": "g1", "data": [{"type": "betaTesters", "id": "t1"}]}),
        )
        .await
        .unwrap();
    assert_eq!(added, "{}");
    let members = server
        .invoke_read_only(
            "betaGroups_betaTesters_getToManyRelated",
            json!({"id": "g1"}),
        )
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(members["data"][0]["attributes"]["email"], "qa@example.com");
}

#[tokio::test]
async fn subscription_lookup_by_app() {
    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/apps/app-1/subscriptionGroups"))
        .and(query_param("include", "subscriptions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"type": "subscriptionGroups", "id": "grp",
                      "attributes": {"referenceName": "Premium"},
                      "relationships": {"subscriptions": {"data": [{"type": "subscriptions", "id": "sub"}]}}}],
            "included": [{"type": "subscriptions", "id": "sub",
                          "attributes": {"productId": "com.example.premium.monthly", "state": "APPROVED"}}]
        })))
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["monetization"]));

    let groups = server
        .invoke_read_only(
            "apps_subscriptionGroups_getToManyRelated",
            json!({"id": "app-1", "include": ["subscriptions"]}),
        )
        .await
        .unwrap()
        .structured
        .unwrap();
    assert_eq!(
        groups["included"][0]["attributes"]["productId"],
        "com.example.premium.monthly"
    );
}

#[tokio::test]
async fn apple_validation_errors_are_preserved() {
    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .respond_with(
            ResponseTemplate::new(400)
                .insert_header("x-request-id", "REQ-123")
                .set_body_json(json!({"errors": [{
                    "id": "e1", "status": "400", "code": "PARAMETER_ERROR.INVALID",
                    "title": "A parameter has an invalid value",
                    "detail": "'nope' is not a valid field name",
                    "source": {"parameter": "fields[apps]"}
                }]})),
        )
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["core"]));

    let result = server
        .invoke_read_only("apps_getCollection", json!({"fields[apps]": ["nope"]}))
        .await
        .unwrap();
    assert!(result.is_error);
    assert_eq!(result.meta["http.x-request-id"], "REQ-123");
    let structured = result.structured.clone().unwrap();
    assert_eq!(structured["status"], 400);
    let error = &structured["body"]["errors"][0];
    assert_eq!(error["code"], "PARAMETER_ERROR.INVALID");
    assert_eq!(error["source"]["parameter"], "fields[apps]");
    assert!(result.into_text().contains("not a valid field name"));
}

#[tokio::test]
async fn requests_carry_an_app_store_connect_jwt() {
    let rng = ring::rand::SystemRandom::new();
    let pkcs8 = ring::signature::EcdsaKeyPair::generate_pkcs8(
        &ring::signature::ECDSA_P256_SHA256_FIXED_SIGNING,
        &rng,
    )
    .unwrap();
    let dir = tempfile::tempdir().unwrap();
    let key_path = dir.path().join("AuthKey_TEST.p8");
    std::fs::write(
        &key_path,
        format!(
            "-----BEGIN PRIVATE KEY-----\n{}\n-----END PRIVATE KEY-----\n",
            STANDARD.encode(pkcs8.as_ref())
        ),
    )
    .unwrap();
    // Unique names nothing else reads; restored when the test ends.
    // (The core config tests serialize their own env mutations.)
    let _env = EnvGuard::set(&[
        ("ASC_JWT_TEST_KEY_ID", "KEY123"),
        ("ASC_JWT_TEST_ISSUER", "issuer-1"),
        ("ASC_JWT_TEST_KEY_PATH", key_path.to_str().unwrap()),
    ]);

    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": []})))
        .expect(1)
        .mount(&api)
        .await;
    let mut config = config(&api.uri(), &["core"]);
    config.auth = AuthConfig::AppStoreConnect {
        key_id_env: "ASC_JWT_TEST_KEY_ID".to_string(),
        issuer_id_env: "ASC_JWT_TEST_ISSUER".to_string(),
        private_key_path_env: "ASC_JWT_TEST_KEY_PATH".to_string(),
    };
    server(config)
        .invoke_read_only("apps_getCollection", json!({}))
        .await
        .unwrap();

    let request = &api.received_requests().await.unwrap()[0];
    let bearer = request.headers["authorization"].to_str().unwrap();
    let token = bearer.strip_prefix("Bearer ").unwrap();
    let parts: Vec<&str> = token.split('.').collect();
    assert_eq!(parts.len(), 3);
    let header: Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[0]).unwrap()).unwrap();
    let claims: Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(parts[1]).unwrap()).unwrap();
    assert_eq!(header["alg"], "ES256");
    assert_eq!(header["kid"], "KEY123");
    assert_eq!(claims["iss"], "issuer-1");
    assert_eq!(claims["aud"], "appstoreconnect-v1");
}

// --- integration: asset upload -------------------------------------------

fn upload_server(api: &MockServer, media_root: &std::path::Path) -> McpProxyServer {
    let mut config = config(&api.uri(), &["assets", "monetization"]);
    config.media_root = Some(media_root.to_path_buf());
    let mut custom = super::build_custom_tools(&config);
    // Same tool, faster polling for tests.
    let upload = custom
        .iter_mut()
        .find(|tool| tool.name == "asset_upload")
        .unwrap();
    upload.handler = Arc::new(AssetUpload {
        http: Ok(reqwest::Client::new()),
        media_root: config.media_root.clone(),
        kinds: enabled_kinds(&config),
        poll_interval: Duration::from_millis(10),
    });
    McpProxyServer::builder(config)
        .tool_profiles(&crate::tools::build_tool_profiles())
        .unwrap()
        .tools(&generated())
        .unwrap()
        .custom_tools(&custom)
        .unwrap()
        .build()
        .unwrap()
}

fn md5_hex(bytes: &[u8]) -> String {
    Md5::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

async fn mount_reservation(api: &MockServer, file: &[u8]) {
    let split = file.len() / 2;
    Mock::given(method("POST"))
        .and(path("/v1/appScreenshots"))
        .and(body_partial_json(json!({"data": {
            "type": "appScreenshots",
            "attributes": {"fileName": "home.png", "fileSize": file.len()},
            "relationships": {"appScreenshotSet": {"data": {"type": "appScreenshotSets", "id": "set-1"}}}
        }})))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"data": {
            "type": "appScreenshots", "id": "shot-1",
            "attributes": {
                "fileName": "home.png",
                "assetDeliveryState": {"state": "AWAITING_UPLOAD"},
                "uploadOperations": [
                    {"method": "PUT", "url": format!("{}/upload/part1", api.uri()),
                     "offset": 0, "length": split,
                     "requestHeaders": [{"name": "Content-Type", "value": "image/png"}]},
                    {"method": "PUT", "url": format!("{}/upload/part2", api.uri()),
                     "offset": split, "length": file.len() - split,
                     "requestHeaders": [{"name": "Content-Type", "value": "image/png"}]}
                ]
            }
        }})))
        .expect(1)
        .mount(api)
        .await;
}

#[tokio::test]
async fn screenshot_upload_runs_reserve_upload_commit_and_processing() {
    let media = tempfile::tempdir().unwrap();
    let file: Vec<u8> = (0u8..=250).cycle().take(10_001).collect();
    std::fs::write(media.path().join("home.png"), &file).unwrap();
    let api = MockServer::start().await;
    mount_reservation(&api, &file).await;
    let split = file.len() / 2;
    Mock::given(method("PUT"))
        .and(path("/upload/part1"))
        .and(header("content-type", "image/png"))
        .and(body_bytes(file[..split].to_vec()))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&api)
        .await;
    Mock::given(method("PUT"))
        .and(path("/upload/part2"))
        .and(body_bytes(file[split..].to_vec()))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&api)
        .await;
    Mock::given(method("PATCH"))
        .and(path("/v1/appScreenshots/shot-1"))
        .and(body_partial_json(json!({"data": {
            "type": "appScreenshots", "id": "shot-1",
            "attributes": {"uploaded": true, "sourceFileChecksum": md5_hex(&file)}
        }})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {
            "type": "appScreenshots", "id": "shot-1",
            "attributes": {"assetDeliveryState": {"state": "UPLOAD_COMPLETE"}}
        }})))
        .expect(1)
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/appScreenshots/shot-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {
            "type": "appScreenshots", "id": "shot-1",
            "attributes": {"assetDeliveryState": {"state": "COMPLETE", "errors": []}}
        }})))
        .mount(&api)
        .await;

    let server = upload_server(&api, media.path());
    let result: Value = serde_json::from_str(
        &server
            .invoke_tool(
                "asset_upload",
                json!({"kind": "appScreenshot", "parentId": "set-1", "file": "home.png"}),
            )
            .await
            .unwrap(),
    )
    .unwrap();

    assert_eq!(result["ok"], true);
    assert_eq!(result["stage"], "complete");
    assert_eq!(result["asset"]["assetId"], "shot-1");
    assert_eq!(result["asset"]["uploadOperations"], 2);
    assert_eq!(result["asset"]["sourceFileChecksum"], md5_hex(&file));
}

#[tokio::test]
async fn failed_upload_names_the_stage_and_keeps_the_reservation() {
    let media = tempfile::tempdir().unwrap();
    let file = vec![7u8; 100];
    std::fs::write(media.path().join("home.png"), &file).unwrap();
    let api = MockServer::start().await;
    mount_reservation(&api, &file).await;
    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(403).set_body_string("signature expired"))
        .mount(&api)
        .await;

    let server = upload_server(&api, media.path());
    let error = server
        .invoke_tool(
            "asset_upload",
            json!({"kind": "appScreenshot", "parentId": "set-1", "file": "home.png"}),
        )
        .await
        .unwrap_err()
        .to_string();
    let failure: Value = serde_json::from_str(&error).unwrap();
    assert_eq!(failure["stage"], "upload");
    assert_eq!(failure["details"]["assetId"], "shot-1");
    assert!(failure["details"]["error"]
        .as_str()
        .unwrap()
        .contains("signature expired"));
    // No commit was attempted.
    let requests = api.received_requests().await.unwrap();
    assert!(requests
        .iter()
        .all(|request: &Request| request.method.as_str() != "PATCH"));
}

#[tokio::test]
async fn upload_rejects_paths_outside_the_media_root() {
    let media = tempfile::tempdir().unwrap();
    let api = MockServer::start().await;
    let server = upload_server(&api, media.path());
    let error = server
        .invoke_tool(
            "asset_upload",
            json!({"kind": "appScreenshot", "parentId": "set-1", "file": "../etc/passwd"}),
        )
        .await
        .unwrap_err()
        .to_string();
    let failure: Value = serde_json::from_str(&error).unwrap();
    assert_eq!(failure["stage"], "validate");
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn reserve_errors_carry_apple_details() {
    let media = tempfile::tempdir().unwrap();
    std::fs::write(media.path().join("home.png"), b"png").unwrap();
    let api = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/appScreenshots"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({"errors": [{
            "status": "409", "code": "ENTITY_ERROR.RELATIONSHIP.INVALID",
            "title": "The provided entity includes a relationship with an invalid value",
            "detail": "screenshot set is full",
            "source": {"pointer": "/data/relationships/appScreenshotSet"}
        }]})))
        .mount(&api)
        .await;
    let server = upload_server(&api, media.path());
    let error = server
        .invoke_tool(
            "asset_upload",
            json!({"kind": "appScreenshot", "parentId": "set-1", "file": "home.png"}),
        )
        .await
        .unwrap_err()
        .to_string();
    let failure: Value = serde_json::from_str(&error).unwrap();
    assert_eq!(failure["stage"], "reserve");
    assert_eq!(
        failure["details"]["upstream"]["body"]["errors"][0]["code"],
        "ENTITY_ERROR.RELATIONSHIP.INVALID"
    );
}

// --- integration: analytics ----------------------------------------------

#[tokio::test]
async fn analytics_segment_download_verifies_and_decompresses() {
    use std::io::Write;
    let csv = "Date,App Name,Installs\n2026-10-01,Example,12\n";
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(csv.as_bytes()).unwrap();
    let gz = encoder.finish().unwrap();

    let api = MockServer::start().await;
    for (id, checksum) in [("seg-ok", md5_hex(&gz)), ("seg-bad", "0".repeat(32))] {
        Mock::given(method("GET"))
            .and(path(format!("/v1/analyticsReportSegments/{id}")))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {
                "type": "analyticsReportSegments", "id": id,
                "attributes": {"checksum": checksum, "sizeInBytes": gz.len(),
                               "url": format!("{}/download/{id}.csv.gz", api.uri())}
            }})))
            .mount(&api)
            .await;
    }
    Mock::given(method("GET"))
        .and(wiremock::matchers::path_regex("^/download/"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(gz.clone()))
        .mount(&api)
        .await;
    let server = server(config(&api.uri(), &["reports"]));

    let ok: Value = serde_json::from_str(
        &server
            .invoke_tool("analytics_segment_download", json!({"segmentId": "seg-ok"}))
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(ok["content"], csv);
    assert_eq!(ok["checksumVerified"], true);
    assert_eq!(ok["truncated"], false);

    let bad: Value = serde_json::from_str(
        &server
            .invoke_tool(
                "analytics_segment_download",
                json!({"segmentId": "seg-bad"}),
            )
            .await
            .unwrap_err()
            .to_string(),
    )
    .unwrap();
    assert_eq!(bad["stage"], "verify");
}

#[test]
fn asset_kinds_follow_enabled_profiles_and_their_tools_exist() {
    fn kinds(profiles: &[&str]) -> Vec<String> {
        let config = config("http://127.0.0.1:9", profiles);
        let custom = super::build_custom_tools(&config);
        let upload = custom
            .iter()
            .find(|tool| tool.name == "asset_upload")
            .unwrap();
        serde_json::from_value(upload.input_schema["properties"]["kind"]["enum"].clone()).unwrap()
    }
    let default = kinds(&["core", "metadata", "assets", "testflight"]);
    assert!(default.contains(&"appScreenshot".to_string()));
    assert!(!default.contains(&"subscriptionAppStoreReviewScreenshot".to_string()));
    assert_eq!(
        kinds(&["monetization"]),
        [
            "subscriptionAppStoreReviewScreenshot",
            "inAppPurchaseAppStoreReviewScreenshot"
        ]
    );
    assert_eq!(kinds(&["all"]).len(), 6);

    // Every generated tool a kind calls is registered whenever the kind is
    // offered, whatever single profile enables it.
    for profile in ["assets", "monetization"] {
        let names = server_names(&[profile]);
        for name in kinds(&[profile]) {
            let kind = KINDS.iter().find(|kind| kind.name == name).unwrap();
            for tool in [kind.create_tool, kind.update_tool, kind.get_tool] {
                assert!(
                    names.contains(tool),
                    "{name} needs {tool} in profile {profile}"
                );
            }
        }
    }

    // asset_upload is assigned to exactly the profiles its kinds need, so it
    // is never exposed without at least one usable kind.
    let declared: BTreeSet<&str> = KINDS.iter().map(|kind| kind.profile).collect();
    let assigned: BTreeSet<&str> = profiles()["asset_upload"].iter().copied().collect();
    assert_eq!(assigned, declared);
}

#[tokio::test]
async fn disabled_kind_is_rejected_before_any_request() {
    let media = tempfile::tempdir().unwrap();
    std::fs::write(media.path().join("shot.png"), b"png").unwrap();
    let api = MockServer::start().await;
    let mut config = config(&api.uri(), &["assets"]);
    config.media_root = Some(media.path().to_path_buf());
    let error = server(config)
        .invoke_tool(
            "asset_upload",
            json!({"kind": "subscriptionAppStoreReviewScreenshot", "parentId": "s", "file": "shot.png"}),
        )
        .await
        .unwrap_err()
        .to_string();
    // The schema enum already excludes the kind, so input validation
    // rejects it before the handler runs.
    assert!(
        error.starts_with("validation error: invalid arguments for asset_upload"),
        "{error}"
    );
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn analytics_download_enforces_size_without_content_length() {
    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/analyticsReportSegments/big"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {
            "type": "analyticsReportSegments", "id": "big",
            "attributes": {"url": format!("{}/download/big.csv", api.uri())}
        }})))
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/download/big.csv"))
        .respond_with(ResponseTemplate::new(200).set_body_string("x,y\n1,2\n"))
        .mount(&api)
        .await;
    let ok: Value = serde_json::from_str(
        &server(config(&api.uri(), &["reports"]))
            .invoke_tool(
                "analytics_segment_download",
                json!({"segmentId": "big", "maxChars": 4}),
            )
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(ok["content"], "x,y\n");
    assert_eq!(ok["truncated"], true);
    assert_eq!(ok["checksumVerified"], false);
}

#[tokio::test]
async fn transfer_client_refuses_redirects_to_plain_http() {
    let api = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/analyticsReportSegments/r"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data": {
            "type": "analyticsReportSegments", "id": "r",
            "attributes": {"url": format!("{}/download/r.csv", api.uri())}
        }})))
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/download/r.csv"))
        .respond_with(
            ResponseTemplate::new(302).insert_header("location", "http://example.com/leak.csv"),
        )
        .mount(&api)
        .await;
    let failure: Value = serde_json::from_str(
        &server(config(&api.uri(), &["reports"]))
            .invoke_tool("analytics_segment_download", json!({"segmentId": "r"}))
            .await
            .unwrap_err()
            .to_string(),
    )
    .unwrap();
    assert_eq!(failure["stage"], "download");
    assert!(
        failure["message"].as_str().unwrap().contains("redirect"),
        "{failure}"
    );
}

#[test]
fn custom_tool_hints() {
    let config = config("http://127.0.0.1:9", &["all"]);
    let custom = super::build_custom_tools(&config);
    let upload = custom
        .iter()
        .find(|tool| tool.name == "asset_upload")
        .unwrap();
    assert_eq!(upload.hints.read_only, Some(false));
    assert_eq!(upload.hints.idempotent, Some(false));
    let download = custom
        .iter()
        .find(|tool| tool.name == "analytics_segment_download")
        .unwrap();
    assert_eq!(download.hints.read_only, Some(true));
}
