//! Partial-write reports preserve upstream failures; retries read authoritative state.
use mcp_factory_core::{McpProxyServer, ProxyConfig};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use wiremock::matchers::{body_json, method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const CODE: &str = "ENTITY_ERROR.ATTRIBUTE.INVALID";
fn rejected() -> ResponseTemplate {
    ResponseTemplate::new(409).set_body_json(json!({"errors":[{"status":"409","code":CODE,"title":"Invalid value","detail":"second planned change rejected","source":{"pointer":"/data/attributes/description"}}]}))
}
fn server(mock: &MockServer) -> McpProxyServer {
    super::test_support::server(ProxyConfig {
        base_url: mock.uri(),
        profiles: Some(vec!["workflow".into()]),
        ..Default::default()
    })
}
async fn app(mock: &MockServer) {
    Mock::given(method("GET")).and(path("/v1/apps")).and(query_param("filter[bundleId]","com.example"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"app","type":"apps","attributes":{"bundleId":"com.example","primaryLocale":"en-US"}}]}))).mount(mock).await;
}
fn assert_progress(error: &Value, planned: &[Value]) {
    assert_eq!(error["stage"], "apply");
    assert_eq!(error["ok"], false);
    assert_eq!(
        error["details"]["progress"]["plannedCount"],
        json!(planned.len())
    );
    assert_eq!(
        error["details"]["progress"]["applied"],
        json!(&planned[..1])
    );
    assert_eq!(
        error["details"]["progress"]["pending"],
        json!(&planned[1..])
    );
    let upstream = &error["details"]["upstream"]["details"]["upstream"];
    assert_eq!(upstream["status"], 409);
    assert_eq!(upstream["body"]["errors"][0]["code"], CODE);
    assert_eq!(
        upstream["body"]["errors"][0]["detail"],
        "second planned change rejected"
    );
    assert_eq!(
        upstream["body"]["errors"][0]["source"]["pointer"],
        "/data/attributes/description"
    );
}
#[tokio::test]
async fn metadata_partial_apply_preserves_apple_error_and_retry_only_writes_pending_diff() {
    let mock = MockServer::start().await;
    app(&mock).await;
    Mock::given(method("GET")).and(path("/v1/apps/app/appStoreVersions"))
        .and(query_param("filter[versionString]","1.0")).and(query_param("filter[platform]","IOS"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"version","type":"appStoreVersions","attributes":{"versionString":"1.0","platform":"IOS"}}]}))).mount(&mock).await;
    let descriptions = Arc::new(Mutex::new([
        "old English".to_string(),
        "old French".to_string(),
    ]));
    let reads = descriptions.clone();
    Mock::given(method("GET")).and(path("/v1/appStoreVersions/version/appStoreVersionLocalizations"))
        .respond_with(move |_:&Request| { let attrs=reads.lock().unwrap(); ResponseTemplate::new(200).set_body_json(json!({"data":[
            {"id":"en","type":"appStoreVersionLocalizations","attributes":{"locale":"en-US","description":attrs[0]}},
            {"id":"fr","type":"appStoreVersionLocalizations","attributes":{"locale":"fr-FR","description":attrs[1]}}
        ]})) }).mount(&mock).await;
    let writes = Arc::new(Mutex::new(Vec::<String>::new()));
    for (id, index) in [("en", 0usize), ("fr", 1usize)] {
        let attrs = descriptions.clone();
        let seen = writes.clone();
        Mock::given(method("PATCH")).and(path(format!("/v1/appStoreVersionLocalizations/{id}")))
            .and(body_json(json!({"data":{"type":"appStoreVersionLocalizations","id":id,"attributes":{"description":if index==0{"new English"}else{"new French"}}}})))
            .respond_with(move |request:&Request| {
                let mut seen=seen.lock().unwrap();let first_french=!seen.iter().any(|x|x=="fr");seen.push(id.into());
                if id=="fr" && first_french {return rejected();}
                let body:Value=serde_json::from_slice(&request.body).unwrap();attrs.lock().unwrap()[index]=body["data"]["attributes"]["description"].as_str().unwrap().to_string();
                ResponseTemplate::new(200).set_body_json(body)
            }).mount(&mock).await;
    }
    let srv = server(&mock);
    let mut args = json!({"bundleId":"com.example","version":"1.0","localizations":{"en-US":{"description":"new English"},"fr-FR":{"description":"new French"}},"dryRun":true});
    let dry: Value = serde_json::from_str(
        &srv.invoke_tool("metadata_apply", args.clone())
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(dry["changes"].as_array().unwrap().len(), 2);
    assert!(writes.lock().unwrap().is_empty());
    args["dryRun"] = json!(false);
    let error: Value = serde_json::from_str(
        &srv.invoke_tool("metadata_apply", args.clone())
            .await
            .unwrap_err()
            .to_string(),
    )
    .unwrap();
    assert_progress(
        &error,
        &[
            json!({"action":"update","resource":"localization","locale":"en-US","fields":["description"]}),
            json!({"action":"update","resource":"localization","locale":"fr-FR","fields":["description"]}),
        ],
    );
    assert_eq!(*descriptions.lock().unwrap(), ["new English", "old French"]);
    let retry: Value =
        serde_json::from_str(&srv.invoke_tool("metadata_apply", args).await.unwrap()).unwrap();
    assert_eq!(retry["unchanged"], json!(["en-US"]));
    assert_eq!(retry["changes"], error["details"]["progress"]["pending"]);
    assert_eq!(*writes.lock().unwrap(), vec!["en", "fr", "fr"]);
    assert_eq!(*descriptions.lock().unwrap(), ["new English", "new French"]);
}

#[tokio::test]
async fn testflight_partial_apply_reports_confirmed_groups_and_retry_only_assigns_pending_group() {
    let mock = MockServer::start().await;
    app(&mock).await;
    Mock::given(method("GET")).and(path("/v1/builds")).and(query_param("filter[app]","app")).and(query_param("filter[version]","42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[{"id":"build","type":"builds","attributes":{"version":"42","processingState":"VALID"}}]}))).mount(&mock).await;
    Mock::given(method("GET")).and(path("/v1/apps/app/betaGroups"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"data":[
            {"id":"g1","type":"betaGroups","attributes":{"name":"First","hasAccessToAllBuilds":false}},
            {"id":"g2","type":"betaGroups","attributes":{"name":"Second","hasAccessToAllBuilds":false}}
        ]}))).mount(&mock).await;
    let assigned = Arc::new(Mutex::new(Vec::<String>::new()));
    let writes = Arc::new(Mutex::new(Vec::<String>::new()));
    for id in ["g1", "g2"] {
        let state = assigned.clone();
        Mock::given(method("GET")).and(path(format!("/v1/betaGroups/{id}/relationships/builds")))
            .respond_with(move |_:&Request| ResponseTemplate::new(200).set_body_json(json!({"data":if state.lock().unwrap().iter().any(|x|x==id){json!([{"type":"builds","id":"build"}])}else{json!([])}}))).mount(&mock).await;
        let state = assigned.clone();
        let seen = writes.clone();
        Mock::given(method("POST"))
            .and(path(format!("/v1/betaGroups/{id}/relationships/builds")))
            .and(body_json(json!({"data":[{"type":"builds","id":"build"}]})))
            .respond_with(move |_: &Request| {
                let mut seen = seen.lock().unwrap();
                let first_second = !seen.iter().any(|x| x == "g2");
                seen.push(id.into());
                if id == "g2" && first_second {
                    return rejected();
                }
                state.lock().unwrap().push(id.into());
                ResponseTemplate::new(204)
            })
            .mount(&mock)
            .await;
    }
    let srv = server(&mock);
    let mut args = json!({"bundleId":"com.example","buildNumber":"42","groupNames":["First","Second"],"dryRun":true});
    srv.invoke_tool("testflight_distribute", args.clone())
        .await
        .unwrap();
    assert!(writes.lock().unwrap().is_empty());
    args["dryRun"] = json!(false);
    let error: Value = serde_json::from_str(
        &srv.invoke_tool("testflight_distribute", args.clone())
            .await
            .unwrap_err()
            .to_string(),
    )
    .unwrap();
    assert_progress(
        &error,
        &[
            json!({"action":"assign","resource":"betaGroupBuild","groupName":"First","buildId":"build"}),
            json!({"action":"assign","resource":"betaGroupBuild","groupName":"Second","buildId":"build"}),
        ],
    );
    assert_eq!(*assigned.lock().unwrap(), vec!["g1"]);
    let retry: Value = serde_json::from_str(
        &srv.invoke_tool("testflight_distribute", args)
            .await
            .unwrap(),
    )
    .unwrap();
    assert_eq!(retry["unchanged"], json!(["First"]));
    assert_eq!(retry["changes"], error["details"]["progress"]["pending"]);
    assert_eq!(*writes.lock().unwrap(), vec!["g1", "g2", "g2"]);
    assert_eq!(*assigned.lock().unwrap(), vec!["g1", "g2"]);
}
