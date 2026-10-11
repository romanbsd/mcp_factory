//! Explicit multi-page fixtures exercise real service traversal and stopping behavior.
use super::{
    screenshots::{self, ScreenshotAction},
    services::Asc,
    testflight::{self, TestflightAction},
};
use mcp_factory_core::{async_trait, OperationInvoker, ProxyConfig, ProxyError, ToolResult};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
struct Pages {
    calls: Mutex<Vec<(String, Value)>>,
    totals: bool,
}
#[async_trait]
impl OperationInvoker for Pages {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.calls
            .lock()
            .unwrap()
            .push((name.to_owned(), args.clone()));
        let mut response = match name {
            "apps_getCollection" => {
                json!({"data":[{"id":"app","attributes":{"bundleId":"example.app"}}]})
            }
            "apps_appStoreVersions_getToManyRelated" => {
                json!({"data":[{"id":"version","attributes":{"versionString":"1","platform":"IOS"}}]})
            }
            "builds_getCollection" => {
                json!({"data":[{"id":"build","attributes":{"version":"2","processingState":"VALID"}}]})
            }
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated" => {
                json!({"data":[{"id":"loc","attributes":{"locale":"en-US"}}]})
            }
            "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated" => {
                json!({"data":[{"id":"set","attributes":{"screenshotDisplayType":"APP_IPHONE_67"}}]})
            }
            "appScreenshotSets_appScreenshots_getToManyRelated" => {
                let start = if args["cursor"] == "next" { 2 } else { 0 };
                let rows=(start..if start==0 {2}else{3}).map(|i|json!({"id":format!("shot{i}"),"attributes":{"fileName":"x".repeat(11_000),"fileSize":42,"sourceFileChecksum":"hash","assetDeliveryState":{"state":"COMPLETE"}}})).collect::<Vec<_>>();
                let mut r = json!({"data":rows});
                if start == 0 {
                    r["links"] = json!({"next":"https://api.appstoreconnect.apple.com/v1/shots?cursor=next"});
                }
                r
            }
            "apps_betaGroups_getToManyRelated" => {
                let start = args["cursor"]
                    .as_str()
                    .map(|s| s.parse::<usize>().unwrap())
                    .unwrap_or(0);
                let rows=(start..(start+25).min(60)).map(|i|json!({"id":format!("g{i}"),"attributes":{"name":format!("Group{i}"),"hasAccessToAllBuilds":true,"isInternalGroup":true}})).collect::<Vec<_>>();
                let mut r = json!({"data":rows});
                if start + 25 < 60 {
                    r["links"] = json!({"next":format!("https://api.appstoreconnect.apple.com/v1/groups?cursor={}",start+25)});
                }
                if self.totals {
                    r["meta"] = json!({"paging":{"total":60}});
                }
                r
            }
            "betaGroups_builds_getToManyRelationship" => {
                json!({"data":[{"id":"build","type":"builds"}]})
            }
            "builds_buildBetaDetail_getToOneRelated" => {
                json!({"data":{"id":"build","attributes":{"internalBuildState":"READY_FOR_BETA_TESTING"}}})
            }
            "cycle" => {
                json!({"data":[{"id":"one"}],"links":{"next":"https://api.appstoreconnect.apple.com/v1/cycle?cursor=same"}})
            }
            _ => panic!("unexpected read {name}"),
        };
        Ok(ToolResult::text("").with_structured(Some(std::mem::take(&mut response))))
    }
    async fn invoke_mutating(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
        panic!("discovery must never write")
    }
}
fn api(totals: bool) -> Pages {
    Pages {
        calls: Mutex::new(vec![]),
        totals,
    }
}
#[tokio::test]
async fn screenshots_stop_before_next_apple_page_and_offsets_keep_positions() {
    let api = api(true);
    let client = Asc { invoker: &api };
    let upload = super::asset_upload::AssetUpload::new(
        Err("unused".into()),
        &ProxyConfig::default(),
        Arc::new(tokio::sync::Semaphore::new(2)),
    );
    let mut args = json!({"bundleId":"example.app","version":"1"});
    let first = screenshots::execute(&client, ScreenshotAction::Get, &args, &upload)
        .await
        .unwrap();
    assert_eq!(first["screenshots"][0]["assets"][0]["id"], "shot0");
    assert_eq!(first["nextOffset"], 1);
    assert_eq!(
        api.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(n, _)| n == "appScreenshotSets_appScreenshots_getToManyRelated")
            .count(),
        1
    );
    args["offset"] = first["nextOffset"].clone();
    let second = screenshots::execute(&client, ScreenshotAction::Get, &args, &upload)
        .await
        .unwrap();
    assert_eq!(second["screenshots"][0]["assets"][0]["id"], "shot1");
    assert_eq!(second["screenshots"][0]["assets"][0]["position"], 1);
    assert_eq!(second["nextOffset"], 2);
    args["offset"] = second["nextOffset"].clone();
    let third = screenshots::execute(&client, ScreenshotAction::Get, &args, &upload)
        .await
        .unwrap();
    assert_eq!(third["screenshots"][0]["assets"][0]["id"], "shot2");
    assert_eq!(third["screenshots"][0]["assets"][0]["position"], 2);
    assert_eq!(third["hasMore"], false);
}
#[tokio::test]
async fn testflight_window_does_not_collect_later_group_pages() {
    let api = api(true);
    let client = Asc { invoker: &api };
    let mut args = json!({"bundleId":"example.app","buildNumber":"2"});
    let first = testflight::execute(&client, TestflightAction::Status, &args)
        .await
        .unwrap();
    assert_eq!(first["groupCount"], 60);
    assert_eq!(first["groups"][0]["id"], "g0");
    assert_eq!(first["groups"][24]["id"], "g24");
    assert_eq!(first["nextOffset"], 25);
    assert_eq!(
        api.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(n, _)| n == "apps_betaGroups_getToManyRelated")
            .count(),
        1
    );
    api.calls.lock().unwrap().clear();
    args["offset"] = json!(50);
    let last = testflight::execute(&client, TestflightAction::Status, &args)
        .await
        .unwrap();
    assert_eq!(last["groups"][0]["id"], "g50");
    assert_eq!(last["groups"].as_array().unwrap().len(), 10);
    assert_eq!(last["nextOffset"], Value::Null);
    assert_eq!(
        api.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|(n, _)| n == "betaGroups_builds_getToManyRelationship")
            .count(),
        10
    );
}
#[tokio::test]
async fn missing_paging_total_is_explicitly_unknown_until_exhausted() {
    let api = api(false);
    let client = Asc { invoker: &api };
    let out = testflight::execute(
        &client,
        TestflightAction::Status,
        &json!({"bundleId":"example.app","buildNumber":"2"}),
    )
    .await
    .unwrap();
    assert_eq!(out["groupCount"], Value::Null);
    assert_eq!(out["groupCountLowerBound"], 25);
    assert_eq!(out["nextOffset"], 25);
}
#[tokio::test]
async fn collection_rejects_repeated_cursor_without_fetching_forever() {
    let api = api(true);
    let client = Asc { invoker: &api };
    let mut collection = client.collection("cycle", json!({}));
    assert_eq!(
        collection.next().await.unwrap().unwrap().rows[0]["id"],
        "one"
    );
    let error = collection.next().await.err().unwrap();
    assert_eq!(
        error.structured.unwrap()["message"],
        "repeated pagination cursor"
    );
    assert_eq!(api.calls.lock().unwrap().len(), 2);
}
