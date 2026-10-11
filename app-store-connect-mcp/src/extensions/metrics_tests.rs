//! Comparable complete tasks through public dispatch, using controlled HTTP fixtures.
use mcp_factory_core::{McpProxyServer, ProxyConfig};
use serde_json::{json, Value};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};
fn server(api: &MockServer, profile: &str, media: &std::path::Path) -> McpProxyServer {
    let config = ProxyConfig {
        base_url: api.uri(),
        profiles: Some(vec![profile.into()]),
        media_root: Some(media.into()),
        ..Default::default()
    };
    super::test_support::server(config)
}
async fn fixture() -> (MockServer, tempfile::TempDir) {
    let api = MockServer::start().await;
    let media = tempfile::tempdir().unwrap();
    std::fs::write(media.path().join("a.png"), b"abc").unwrap();
    let url = api.uri();
    Mock::given(|_:&Request|true).respond_with(move|request:&Request|{
        let path=request.url.path();
        let data=match (request.method.as_str(),path) {
            ("GET","/v1/apps")=>json!([{"id":"app","type":"apps","attributes":{"name":"Example","bundleId":"com.example","sku":"example","primaryLocale":"en-US"}}]),
            ("GET","/v1/apps/app/appStoreVersions")=>json!([{"id":"version","type":"appStoreVersions","attributes":{"versionString":"1.0","platform":"IOS","appStoreState":"PREPARE_FOR_SUBMISSION","releaseType":"MANUAL"}}]),
            ("GET","/v1/appStoreVersions/version/build")=>json!({"id":"build","type":"builds","attributes":{"version":"42","processingState":"VALID","expired":false}}),
            ("GET","/v1/appStoreVersions/version/appStoreVersionPhasedRelease")=>Value::Null,
            ("GET","/v1/reviewSubmissions")=>json!([{"id":"review","type":"reviewSubmissions","attributes":{"state":"READY_FOR_REVIEW","platform":"IOS"}}]),
            ("GET","/v1/reviewSubmissions/review/items")=>json!([{"id":"item","type":"reviewSubmissionItems","relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":"version"}}}}]),
            ("GET","/v1/appStoreVersions/version/appStoreVersionLocalizations")=>json!([{"id":"loc","type":"appStoreVersionLocalizations","attributes":{"locale":"en-US","description":"old"}}]),
            ("PATCH","/v1/appStoreVersionLocalizations/loc")=>json!({"id":"loc","type":"appStoreVersionLocalizations","attributes":{"locale":"en-US","description":"new"}}),
            ("GET","/v1/appStoreVersionLocalizations/loc/appScreenshotSets")=>json!([{"id":"set","type":"appScreenshotSets","attributes":{"screenshotDisplayType":"APP_IPHONE_67"}}]),
            ("GET","/v1/appScreenshotSets/set/appScreenshots")=>json!([]),
            ("POST","/v1/appScreenshots")=>json!({"id":"shot","type":"appScreenshots","attributes":{"fileName":"a.png","fileSize":3,"assetDeliveryState":{"state":"AWAITING_UPLOAD"},"uploadOperations":[{"url":format!("{url}/upload"),"method":"PUT","offset":0,"length":3,"requestHeaders":[]}]}}),
            ("PATCH","/v1/appScreenshots/shot")=>json!({"id":"shot","type":"appScreenshots","attributes":{"assetDeliveryState":{"state":"COMPLETE"},"sourceFileChecksum":"900150983cd24fb0d6963f7d28e17f72"}}),
            ("PUT","/upload")=>{assert_eq!(request.body,b"abc");return ResponseTemplate::new(200);},
            ("PATCH","/v1/appScreenshotSets/set/relationships/appScreenshots")=>{assert_eq!(serde_json::from_slice::<Value>(&request.body).unwrap(),json!({"data":[{"id":"shot","type":"appScreenshots"}]}));return ResponseTemplate::new(204);},
            ("GET","/v1/builds")=>json!([{"id":"build","type":"builds","attributes":{"version":"42","processingState":"VALID","expired":false}}]),
            ("GET","/v1/apps/app/betaGroups")=>json!([{"id":"g1","type":"betaGroups","attributes":{"name":"Existing","hasAccessToAllBuilds":false}},{"id":"g2","type":"betaGroups","attributes":{"name":"New","hasAccessToAllBuilds":false}}]),
            ("GET","/v1/betaGroups/g1/relationships/builds")=>json!([{"id":"build","type":"builds"}]),
            ("GET","/v1/betaGroups/g2/relationships/builds")=>json!([]),
            ("POST","/v1/betaGroups/g2/relationships/builds")=>{assert_eq!(serde_json::from_slice::<Value>(&request.body).unwrap(),json!({"data":[{"id":"build","type":"builds"}]}));return ResponseTemplate::new(204);},
            _=>return ResponseTemplate::new(500).set_body_string(format!("Unexpected fixture request {} {path}",request.method)),
        };
        ResponseTemplate::new(200).set_body_json(json!({"data":data}))
    }).mount(&api).await;
    (api, media)
}
async fn calls(server: &McpProxyServer, operations: Vec<(&str, Value)>) -> usize {
    let mut count = 0;
    for (operation, args) in operations {
        server.invoke_tool(operation, args).await.unwrap();
        count += 1;
    }
    count
}
#[tokio::test]
async fn complete_task_call_counts_and_output_bytes() {
    let common = json!({"bundleId":"com.example","version":"1.0"});
    for name in [
        "release_status",
        "metadata_apply",
        "screenshots_sync",
        "testflight_distribute",
    ] {
        let (api, media) = fixture().await;
        let raw = server(&api, "all", media.path());
        let workflow = server(&api, "workflow", media.path());
        let app = (
            "apps_getCollection",
            json!({"filter[bundleId]":["com.example"]}),
        );
        let version = (
            "apps_appStoreVersions_getToManyRelated",
            json!({"id":"app","filter[versionString]":["1.0"],"filter[platform]":["IOS"]}),
        );
        let localization = (
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated",
            json!({"id":"version"}),
        );
        let (baseline, input) = match name {
            "release_status" => (
                vec![
                    app,
                    version,
                    (
                        "appStoreVersions_build_getToOneRelated",
                        json!({"id":"version"}),
                    ),
                    (
                        "appStoreVersions_appStoreVersionPhasedRelease_getToOneRelated",
                        json!({"id":"version"}),
                    ),
                    (
                        "reviewSubmissions_getCollection",
                        json!({"filter[app]":["app"],"filter[platform]":["IOS"]}),
                    ),
                    (
                        "reviewSubmissions_items_getToManyRelated",
                        json!({"id":"review","include":["appStoreVersion"]}),
                    ),
                ],
                common.clone(),
            ),
            "metadata_apply" => (
                vec![
                    app,
                    version,
                    localization,
                    (
                        "appStoreVersionLocalizations_updateInstance",
                        json!({"id":"loc","data":{"id":"loc","type":"appStoreVersionLocalizations","attributes":{"description":"new"}}}),
                    ),
                ],
                json!({"bundleId":"com.example","version":"1.0","localizations":{"en-US":{"description":"new"}}}),
            ),
            "screenshots_sync" => (
                vec![
                    app,
                    version,
                    localization,
                    (
                        "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated",
                        json!({"id":"loc"}),
                    ),
                    (
                        "appScreenshotSets_appScreenshots_getToManyRelated",
                        json!({"id":"set"}),
                    ),
                    (
                        "asset_upload",
                        json!({"kind":"appScreenshot","parentId":"set","file":"a.png","waitSeconds":0}),
                    ),
                    (
                        "appScreenshotSets_appScreenshots_replaceToManyRelationship",
                        json!({"id":"set","data":[{"id":"shot","type":"appScreenshots"}]}),
                    ),
                ],
                json!({"bundleId":"com.example","version":"1.0","locale":"en-US","displayType":"APP_IPHONE_67","files":["a.png"],"waitSeconds":0}),
            ),
            _ => (
                vec![
                    app,
                    (
                        "builds_getCollection",
                        json!({"filter[app]":["app"],"filter[version]":["42"]}),
                    ),
                    ("apps_betaGroups_getToManyRelated", json!({"id":"app"})),
                    (
                        "betaGroups_builds_getToManyRelationship",
                        json!({"id":"g1"}),
                    ),
                    (
                        "betaGroups_builds_getToManyRelationship",
                        json!({"id":"g2"}),
                    ),
                    (
                        "betaGroups_builds_createToManyRelationship",
                        json!({"id":"g2","data":[{"id":"build","type":"builds"}]}),
                    ),
                ],
                json!({"bundleId":"com.example","buildNumber":"42","groupNames":["Existing","New"]}),
            ),
        };
        let before = calls(&raw, baseline).await;
        let output = workflow.invoke_tool(name, input).await.unwrap();
        let value: Value = serde_json::from_str(&output).unwrap();
        match name {
            "release_status" => {
                assert_eq!(value["build"]["processingState"], "VALID");
                assert_eq!(value["reviewSubmissions"][0]["state"], "READY_FOR_REVIEW");
            }
            "metadata_apply" => assert_eq!(
                value["changes"],
                json!([{"action":"update","resource":"localization","locale":"en-US","fields":["description"]}])
            ),
            "screenshots_sync" => {
                assert_eq!(value["pending"], json!([]));
                assert_eq!(value["ok"], true);
            }
            _ => {
                assert_eq!(value["changes"].as_array().unwrap().len(), 1);
                assert_eq!(value["changes"][0]["groupName"], "New");
            }
        }
        println!(
            "TASK_METRIC {}",
            json!({"task":name,"raw_public_calls":before,"workflow_public_calls":1,"workflow_output_bytes":output.len()})
        );
    }
}
