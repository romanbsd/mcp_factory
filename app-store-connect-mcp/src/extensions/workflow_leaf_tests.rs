use super::{metadata, services::Asc, testflight};
use mcp_factory_core::{async_trait, OperationInvoker, ProxyError, ToolResult};
use serde_json::{json, Value};
use std::sync::Mutex;

#[derive(Default)]
struct Fake {
    localizations: Vec<Value>,
    groups: Vec<Value>,
    assigned: Vec<String>,
    writes: Mutex<Vec<(String, Value)>>,
    membership_reads: Mutex<Vec<String>>,
    reads: Mutex<Vec<(String, Value)>>,
}
#[async_trait]
impl OperationInvoker for Fake {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.reads
            .lock()
            .unwrap()
            .push((name.to_owned(), args.clone()));
        let data = match name {
            "apps_getCollection" => {
                json!([{"id":"app","attributes":{"bundleId":"com.example","primaryLocale":"en-US"}}])
            }
            "apps_appStoreVersions_getToManyRelated" => {
                json!([{"id":"version","attributes":{"versionString":"1.0","platform":"IOS"}}])
            }
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated" => {
                json!(self.localizations)
            }
            "builds_getCollection" => {
                json!([{"id":"build","attributes":{"version":"42","processingState":"VALID","expired":false}}])
            }
            "apps_betaGroups_getToManyRelated" => json!(self.groups),
            "betaGroups_builds_getToManyRelationship" => {
                self.membership_reads
                    .lock()
                    .unwrap()
                    .push(args["id"].as_str().unwrap().to_owned());
                if self
                    .assigned
                    .contains(&args["id"].as_str().unwrap().to_owned())
                {
                    json!([{"id":"build","type":"builds"}])
                } else {
                    json!([])
                }
            }
            "builds_buildBetaDetail_getToOneRelated" => {
                json!({"id":"detail","attributes":{"internalBuildState":"IN_BETA_TESTING","externalBuildState":"READY_FOR_BETA_TESTING"}})
            }
            _ => panic!("unexpected read {name}"),
        };
        Ok(ToolResult::text("").with_structured(Some(json!({"data":data}))))
    }
    async fn invoke_mutating(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.writes.lock().unwrap().push((name.into(), args));
        Ok(ToolResult::text("").with_structured(Some(json!({}))))
    }
}

#[tokio::test]
async fn testflight_status_pages_group_inspection_without_reading_later_memberships() {
    let fake = Fake {
        groups: (0..60)
            .map(|n| group(&format!("g{n}"), &format!("Group {n}")))
            .collect(),
        assigned: (0..60).map(|n| format!("g{n}")).collect(),
        ..Default::default()
    };
    let mut args = json!({"bundleId":"com.example","buildNumber":"42"});
    let first = testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Status,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(first["groupCount"], 60);
    assert_eq!(first["scannedGroups"], 25);
    assert_eq!(first["nextOffset"], 25);
    assert_eq!(first["groups"].as_array().unwrap().len(), 25);
    assert!(first.to_string().len() <= 10_000);
    assert_eq!(
        *fake.membership_reads.lock().unwrap(),
        (0..25).map(|n| format!("g{n}")).collect::<Vec<_>>()
    );
    fake.membership_reads.lock().unwrap().clear();
    args["offset"] = first["nextOffset"].clone();
    let second = testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Status,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(second["nextOffset"], 50);
    assert_eq!(second["scannedGroups"], 25);
    assert_eq!(
        *fake.membership_reads.lock().unwrap(),
        (25..50).map(|n| format!("g{n}")).collect::<Vec<_>>()
    );
    fake.membership_reads.lock().unwrap().clear();
    args["offset"] = json!(50);
    let last = testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Status,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(last["nextOffset"], Value::Null);
    assert_eq!(last["scannedGroups"], 10);
    assert_eq!(
        *fake.membership_reads.lock().unwrap(),
        (50..60).map(|n| format!("g{n}")).collect::<Vec<_>>()
    );
    fake.membership_reads.lock().unwrap().clear();
    args["groupNames"] = json!(["Group 59", "Group 3"]);
    args["offset"] = json!(1);
    let selected = testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Status,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(selected["groupCount"], 2);
    assert_eq!(selected["scannedGroups"], 1);
    assert_eq!(selected["nextOffset"], Value::Null);
    assert_eq!(*fake.membership_reads.lock().unwrap(), vec!["g3"]);
    assert!(fake.writes.lock().unwrap().is_empty());
}
fn metadata_args() -> Value {
    json!({"bundleId":"com.example","version":"1.0","localizations":{"en-US":{"description":"same","keywords":"new"},"fr-FR":{"description":"bonjour"}}})
}
fn localized() -> Value {
    json!({"id":"locale","attributes":{"locale":"en-US","description":"same","keywords":"old"}})
}
fn group(id: &str, name: &str) -> Value {
    json!({"id":id,"attributes":{"name":name,"isInternalGroup":false,"hasAccessToAllBuilds":false}})
}
fn flight_args() -> Value {
    json!({"bundleId":"com.example","buildNumber":"42","groupNames":["Existing","New"]})
}

#[tokio::test]
async fn metadata_patches_exact_diff_and_creates_required_relation() {
    let fake = Fake {
        localizations: vec![localized()],
        ..Default::default()
    };
    let report = metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Apply,
        &metadata_args(),
    )
    .await
    .unwrap();
    assert_eq!(report["changes"].as_array().unwrap().len(), 2);
    let writes = fake.writes.lock().unwrap();
    assert_eq!(
        writes[0],
        (
            "appStoreVersionLocalizations_updateInstance".into(),
            json!({"id":"locale","data":{"id":"locale","type":"appStoreVersionLocalizations","attributes":{"keywords":"new"}}})
        )
    );
    assert_eq!(
        writes[1],
        (
            "appStoreVersionLocalizations_createInstance".into(),
            json!({"data":{"type":"appStoreVersionLocalizations","attributes":{"locale":"fr-FR","description":"bonjour"},"relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":"version"}}}}})
        )
    );
}
#[tokio::test]
async fn metadata_dry_run_and_noop_never_write() {
    let fake = Fake {
        localizations: vec![localized()],
        ..Default::default()
    };
    let mut args = metadata_args();
    args["dryRun"] = json!(true);
    let report = metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Apply,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(report["changes"].as_array().unwrap().len(), 2);
    args["localizations"] = json!({"en-US":{"description":"same","keywords":"old"}});
    args["dryRun"] = json!(false);
    let report = metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Apply,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(report["changes"], json!([]));
    assert_eq!(report["unchanged"], json!(["en-US"]));
    assert!(fake.writes.lock().unwrap().is_empty());
}

#[tokio::test]
async fn metadata_explicit_null_clears_only_that_field_and_omission_is_noop() {
    let fake = Fake {
        localizations: vec![localized()],
        ..Default::default()
    };
    let mut args = json!({"bundleId":"com.example","version":"1.0","localizations":{"en-US":{"description":null}}});
    let result = metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Apply,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(
        result["changes"],
        json!([{"action":"update","resource":"localization","locale":"en-US","fields":["description"]}])
    );
    assert_eq!(
        *fake.writes.lock().unwrap(),
        vec![(
            "appStoreVersionLocalizations_updateInstance".to_owned(),
            json!({"id":"locale","data":{"id":"locale","type":"appStoreVersionLocalizations","attributes":{"description":null}}})
        )]
    );
    args["localizations"] = json!({"en-US":{}});
    let result = metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Apply,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(
        result,
        json!({"changes":[],"unchanged":["en-US"],"warnings":[],"dryRun":false})
    );
    assert_eq!(fake.writes.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn localization_selection_preserves_sparse_queries_and_normalized_output() {
    let fake = Fake {
        localizations: vec![
            localized(),
            json!({"id":"other","attributes":{"locale":"fr-FR","description":"bonjour"}}),
        ],
        ..Default::default()
    };
    let result=metadata::execute(&Asc{invoker:&fake},metadata::MetadataAction::Get,&json!({"bundleId":"com.example","version":"1.0","locales":["en-US"],"fields":["description"]})).await.unwrap();
    assert_eq!(
        result["localizations"],
        json!([{"id":"locale","locale":"en-US","description":"same"}])
    );
    let reads = fake.reads.lock().unwrap();
    let query = &reads
        .iter()
        .find(|(operation, _)| {
            operation == "appStoreVersions_appStoreVersionLocalizations_getToManyRelated"
        })
        .unwrap()
        .1;
    assert_eq!(
        query,
        &json!({"id":"version","filter[locale]":["en-US"],"fields[appStoreVersionLocalizations]":["description","locale"],"limit":200})
    );
    assert!(fake.writes.lock().unwrap().is_empty());
}
#[tokio::test]
async fn metadata_oversized_read_fails_without_truncation() {
    let fake = Fake {
        localizations: vec![
            json!({"id":"locale","attributes":{"locale":"en-US","description":"x".repeat(21000)}}),
        ],
        ..Default::default()
    };
    assert!(metadata::execute(
        &Asc { invoker: &fake },
        metadata::MetadataAction::Get,
        &json!({"bundleId":"com.example","version":"1.0"})
    )
    .await
    .is_err());
}
#[tokio::test]
async fn testflight_assigns_only_missing_relationship_and_repeat_is_noop() {
    let mut fake = Fake {
        groups: vec![group("g1", "Existing"), group("g2", "New")],
        assigned: vec!["g1".into()],
        ..Default::default()
    };
    testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Distribute,
        &flight_args(),
    )
    .await
    .unwrap();
    assert_eq!(
        *fake.writes.lock().unwrap(),
        vec![(
            "betaGroups_builds_createToManyRelationship".into(),
            json!({"id":"g2","data":[{"type":"builds","id":"build"}]})
        )]
    );
    fake.assigned.push("g2".into());
    fake.writes.lock().unwrap().clear();
    let report = testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Distribute,
        &flight_args(),
    )
    .await
    .unwrap();
    assert_eq!(report["changes"], json!([]));
    assert!(fake.writes.lock().unwrap().is_empty());
}
#[tokio::test]
async fn testflight_unknown_or_ambiguous_group_never_partially_writes() {
    for groups in [
        vec![group("g1", "Existing")],
        vec![
            group("g1", "Existing"),
            group("g2", "New"),
            group("g3", "New"),
        ],
    ] {
        let fake = Fake {
            groups,
            ..Default::default()
        };
        assert!(testflight::execute(
            &Asc { invoker: &fake },
            testflight::TestflightAction::Distribute,
            &flight_args()
        )
        .await
        .is_err());
        assert!(fake.writes.lock().unwrap().is_empty());
    }
}
#[tokio::test]
async fn testflight_dry_run_does_not_write_and_remove_only_existing() {
    let fake = Fake {
        groups: vec![group("g1", "Existing"), group("g2", "New")],
        assigned: vec!["g1".into()],
        ..Default::default()
    };
    let mut args = flight_args();
    args["dryRun"] = json!(true);
    testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Distribute,
        &args,
    )
    .await
    .unwrap();
    assert!(fake.writes.lock().unwrap().is_empty());
    args["dryRun"] = json!(false);
    testflight::execute(
        &Asc { invoker: &fake },
        testflight::TestflightAction::Remove,
        &args,
    )
    .await
    .unwrap();
    assert_eq!(
        *fake.writes.lock().unwrap(),
        vec![(
            "betaGroups_builds_deleteToManyRelationship".into(),
            json!({"id":"g1","data":[{"type":"builds","id":"build"}]})
        )]
    );
}
