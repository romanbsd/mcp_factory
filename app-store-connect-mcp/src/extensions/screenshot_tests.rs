use super::{
    asset_upload::{AssetUpload, KINDS},
    screenshots::{execute, ScreenshotAction},
    services::Asc,
};
use mcp_factory_core::{async_trait, OperationInvoker, ProxyError, ToolResult};
use serde_json::{json, Value};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::Semaphore;
use wiremock::matchers::{body_bytes, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
const CHECKSUM: &str = "900150983cd24fb0d6963f7d28e17f72";
fn asset(id: &str, name: &str, state: &str, checksum: &str) -> Value {
    json!({"type":"appScreenshots","id":id,"attributes":{"fileName":name,"fileSize":3,"sourceFileChecksum":checksum,"assetDeliveryState":{"state":state}},"relationships":{"appScreenshotSet":{"data":{"type":"appScreenshotSets","id":"set"}}}})
}
struct Api {
    existing: Vec<Value>,
    missing_set: bool,
    upload_url: String,
    completed: bool,
    calls: Mutex<Vec<(String, Value)>>,
}
#[async_trait]
impl OperationInvoker for Api {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.calls.lock().unwrap().push((name.to_string(), args));
        let data = match name {
            "apps_getCollection" => json!([{"id":"app","attributes":{"bundleId":"example.app"}}]),
            "apps_appStoreVersions_getToManyRelated" => {
                json!([{"id":"version","attributes":{"versionString":"1","platform":"IOS"}}])
            }
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated" => {
                json!([{"id":"loc","attributes":{"locale":"en-US"}}])
            }
            "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated" => {
                if self.missing_set {
                    json!([])
                } else {
                    json!([{"id":"set","attributes":{"screenshotDisplayType":"APP_IPHONE_67"}}])
                }
            }
            "appScreenshotSets_appScreenshots_getToManyRelated" => json!(self.existing),
            "appScreenshots_getInstance" => self.existing.first().cloned().unwrap(),
            _ => panic!("unexpected read {name}"),
        };
        Ok(super::success(json!({"data":data})))
    }
    async fn invoke_mutating(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.calls.lock().unwrap().push((name.to_string(), args));
        let data = match name {
            "appScreenshotSets_createInstance" => {
                json!({"id":"set","attributes":{"screenshotDisplayType":"APP_IPHONE_67"}})
            }
            "appScreenshots_createInstance" => {
                let mut row = asset("new", "shot.png", "AWAITING_UPLOAD", CHECKSUM);
                row["attributes"]["uploadOperations"] = if self.upload_url.is_empty() {
                    json!([])
                } else {
                    json!([{"method":"PUT","url":self.upload_url,"offset":0,"length":3,"requestHeaders":[]}])
                };
                row
            }
            "appScreenshots_updateInstance" => asset(
                "new",
                "shot.png",
                if self.completed {
                    "COMPLETE"
                } else {
                    "PROCESSING"
                },
                CHECKSUM,
            ),
            "appScreenshotSets_appScreenshots_replaceToManyRelationship"
            | "appScreenshots_deleteInstance" => Value::Null,
            _ => panic!("unexpected mutation {name}"),
        };
        Ok(super::success(json!({"data":data})))
    }
}
impl Api {
    fn new(existing: Vec<Value>) -> Self {
        Self {
            existing,
            missing_set: false,
            upload_url: String::new(),
            completed: true,
            calls: Mutex::new(Vec::new()),
        }
    }
    fn writes(&self) -> Vec<(String, Value)> {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(name, _)| {
                name.contains("createInstance")
                    || name.contains("updateInstance")
                    || name.contains("replaceToMany")
                    || name.contains("deleteInstance")
            })
            .cloned()
            .collect()
    }
}
fn uploader(root: &std::path::Path) -> AssetUpload {
    AssetUpload {
        http: Ok(reqwest::Client::new()),
        transfers: Arc::new(Semaphore::new(2)),
        media_root: Some(root.to_path_buf()),
        kinds: KINDS.iter().collect(),
        poll_interval: Duration::from_millis(1),
    }
}
fn args() -> Value {
    json!({"bundleId":"example.app","version":"1","locale":"en-US","displayType":"APP_IPHONE_67","waitSeconds":0,"files":["shot.png"]})
}
async fn media() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    tokio::fs::write(dir.path().join("shot.png"), b"abc")
        .await
        .unwrap();
    dir
}
#[tokio::test]
async fn screenshot_exact_complete_match_has_no_writes() {
    let dir = media().await;
    let api = Api::new(vec![asset("same", "shot.png", "COMPLETE", CHECKSUM)]);
    let output = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &args(),
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    assert!(api.writes().is_empty());
    assert_eq!(output["changes"], json!([]));
    assert_eq!(output["unchanged"][0]["id"], "same");
}
#[tokio::test]
async fn screenshot_dry_run_plans_missing_set_without_writes() {
    let dir = media().await;
    let mut api = Api::new(vec![]);
    api.missing_set = true;
    let mut input = args();
    input["dryRun"] = json!(true);
    let output = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    assert!(api.writes().is_empty());
    assert_eq!(output["changes"][0]["resource"], "set");
    assert_eq!(output["changes"][1]["action"], "upload");
    assert_eq!(output["changes"][2]["resource"], "order");
}
#[tokio::test]
async fn screenshot_reservation_failure_returns_checkpoint_without_order_or_delete() {
    let dir = media().await;
    let mut api = Api::new(vec![]);
    api.missing_set = true;
    let err = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &args(),
        &uploader(dir.path()),
    )
    .await
    .unwrap_err();
    let body = err.structured.unwrap();
    assert_eq!(
        body["details"]["result"]["resumes"]["shot.png"]["assetId"],
        "new"
    );
    let names: Vec<_> = api.writes().into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        names,
        [
            "appScreenshotSets_createInstance",
            "appScreenshots_createInstance"
        ]
    );
}
async fn put_server() -> MockServer {
    let http = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/shot"))
        .and(body_bytes(b"abc"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&http)
        .await;
    http
}
#[tokio::test]
async fn screenshot_upload_preserves_existing_assets_by_default() {
    let dir = media().await;
    let http = put_server().await;
    let mut api = Api::new(vec![asset("old", "old.png", "COMPLETE", CHECKSUM)]);
    api.upload_url = format!("{}/shot", http.uri());
    execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &args(),
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    let writes = api.writes();
    assert_eq!(
        writes.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>(),
        [
            "appScreenshots_createInstance",
            "appScreenshots_updateInstance",
            "appScreenshotSets_appScreenshots_replaceToManyRelationship"
        ]
    );
    assert_eq!(
        writes.last().unwrap().1["data"],
        json!([{"type":"appScreenshots","id":"new"},{"type":"appScreenshots","id":"old"}])
    );
}
#[tokio::test]
async fn screenshot_pending_upload_never_orders_or_removes() {
    let dir = media().await;
    let http = put_server().await;
    let mut api = Api::new(vec![asset("old", "old.png", "COMPLETE", CHECKSUM)]);
    api.upload_url = format!("{}/shot", http.uri());
    api.completed = false;
    let mut input = args();
    input["removeObsolete"] = json!(true);
    let out = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    assert_eq!(out["pending"][0]["state"], "PROCESSING");
    assert_eq!(api.writes().len(), 2);
    assert_eq!(out["resumes"]["shot.png"]["assetId"], "new");
}
#[tokio::test]
async fn screenshot_removal_follows_complete_upload_and_order() {
    let dir = media().await;
    let http = put_server().await;
    let mut api = Api::new(vec![asset("old", "old.png", "COMPLETE", CHECKSUM)]);
    api.upload_url = format!("{}/shot", http.uri());
    let mut input = args();
    input["removeObsolete"] = json!(true);
    execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    let names: Vec<_> = api.writes().into_iter().map(|(n, _)| n).collect();
    assert_eq!(
        names,
        [
            "appScreenshots_createInstance",
            "appScreenshots_updateInstance",
            "appScreenshotSets_appScreenshots_replaceToManyRelationship",
            "appScreenshots_deleteInstance"
        ]
    );
}
#[tokio::test]
async fn screenshot_resume_reuses_existing_reservation_and_no_post() {
    let dir = media().await;
    let api = Api::new(vec![asset(
        "pending",
        "shot.png",
        "UPLOAD_COMPLETE",
        CHECKSUM,
    )]);
    let mut input = args();
    input["resumes"] = json!({"shot.png":{"assetId":"pending","kind":"appScreenshot","parentId":"set","file":"shot.png","fileSize":3,"sourceFileChecksum":CHECKSUM,"completedRanges":[],"committed":true}});
    let out = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path()),
    )
    .await
    .unwrap();
    assert!(api.writes().is_empty());
    assert_eq!(out["pending"][0]["id"], "pending");
    assert_eq!(out["resumes"]["shot.png"]["assetId"], "pending");
}
#[tokio::test]
async fn screenshot_validates_all_files_before_creating_set() {
    let dir = media().await;
    let mut api = Api::new(vec![]);
    api.missing_set = true;
    let mut input = args();
    input["files"] = json!(["shot.png", "missing.png"]);
    assert!(execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path())
    )
    .await
    .is_err());
    assert!(api.writes().is_empty());
}

#[tokio::test]
async fn screenshot_get_has_bounded_continuation_and_normalized_position() {
    let api = Api::new(vec![
        asset("one", &"a".repeat(11_000), "COMPLETE", CHECKSUM),
        asset("two", &"b".repeat(11_000), "PROCESSING", CHECKSUM),
    ]);
    let uploader = uploader(std::path::Path::new("/tmp"));
    let first = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Get,
        &args(),
        &uploader,
    )
    .await
    .unwrap();
    assert!(first.to_string().len() < 20_000);
    assert_eq!(first["hasMore"], true);
    assert_eq!(first["nextOffset"], 1);
    assert_eq!(first["screenshots"][0]["assets"][0]["position"], 0);
    let mut next = args();
    next["offset"] = first["nextOffset"].clone();
    let second = execute(
        &Asc { invoker: &api },
        ScreenshotAction::Get,
        &next,
        &uploader,
    )
    .await
    .unwrap();
    assert_eq!(second["hasMore"], false);
    assert_eq!(second["screenshots"][0]["assets"][0]["position"], 1);
    assert_eq!(second["screenshots"][0]["assets"][0]["state"], "PROCESSING");
}
#[tokio::test]
async fn screenshot_duplicate_paths_fail_before_writes() {
    let dir = media().await;
    let api = Api::new(vec![]);
    let mut input = args();
    input["files"] = json!(["shot.png", "shot.png"]);
    assert!(execute(
        &Asc { invoker: &api },
        ScreenshotAction::Sync,
        &input,
        &uploader(dir.path())
    )
    .await
    .is_err());
    assert!(api.writes().is_empty());
}
