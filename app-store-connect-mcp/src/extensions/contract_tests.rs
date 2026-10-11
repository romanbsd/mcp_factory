//! Public surface and generated API contracts, including hidden capabilities.
use super::workflows;
use mcp_factory_core::{ExecutionKind, McpProxyServer, ProxyConfig};
use serde_json::json;
fn server(profile: &str) -> McpProxyServer {
    let config = ProxyConfig {
        profiles: Some(vec![profile.into()]),
        ..Default::default()
    };
    super::test_support::server(config)
}
#[test]
fn workflow_default_is_small_sorted_and_hides_all_generated_dependencies() {
    assert_eq!(crate::tools::default_profiles(), ["workflow"]);
    let srv = server("workflow");
    let names = srv.tool_names();
    assert_eq!(names.len(), 15);
    assert!(names.windows(2).all(|pair| pair[0] < pair[1]));
    // Independent name contract: adding a registry entry must not update this implicitly.
    let expected: std::collections::BTreeSet<_> = [
        "app_get",
        "version_get",
        "release_status",
        "release_prepare",
        "metadata_get",
        "metadata_apply",
        "screenshots_get",
        "screenshots_sync",
        "testflight_status",
        "testflight_distribute",
        "testflight_remove",
        "review_submit",
        "release",
        "asset_upload",
        "report_query",
    ]
    .into_iter()
    .collect();
    assert_eq!(
        names
            .iter()
            .map(String::as_str)
            .collect::<std::collections::BTreeSet<_>>(),
        expected
    );
    assert!(!names.iter().any(|name| crate::tools::operation_catalog()
        .iter()
        .any(|op| op.name == name)));
    let payload = serde_json::to_vec(&json!({"tools":srv.tool_metadata()})).unwrap();
    assert!(
        payload.len() <= 25000,
        "workflow listing grew to {} bytes",
        payload.len()
    );
    let resource = srv
        .read_resource_content("schema://operation/apps_getCollection")
        .unwrap();
    let value: serde_json::Value = serde_json::from_str(&resource).unwrap();
    assert_eq!(value["execution"]["method"], "GET");
    assert!(value["inputSchema"]["properties"]["filter[bundleId]"].is_object());
    assert!(srv
        .read_resource_content("docs://workflow/release")
        .unwrap()
        .contains("review_submit"));
}
#[test]
fn workflow_registry_preserves_safety_hints_and_required_inputs() {
    let cfg = ProxyConfig::default();
    let specs = workflows::tools(std::sync::Arc::new(super::asset_upload::AssetUpload::new(
        Err("unused client".into()),
        &cfg,
        std::sync::Arc::new(tokio::sync::Semaphore::new(2)),
    )));
    let expected: &[(&str, bool, bool, bool, &[&str])] = &[
        ("app_get", true, false, true, &["bundleId"]),
        ("version_get", true, false, true, &["bundleId", "version"]),
        (
            "release_status",
            true,
            false,
            true,
            &["bundleId", "version"],
        ),
        (
            "release_prepare",
            true,
            false,
            true,
            &["bundleId", "version"],
        ),
        ("metadata_get", true, false, true, &["bundleId", "version"]),
        (
            "metadata_apply",
            false,
            false,
            true,
            &["bundleId", "version", "localizations"],
        ),
        (
            "screenshots_get",
            true,
            false,
            true,
            &["bundleId", "version"],
        ),
        (
            "screenshots_sync",
            false,
            true,
            true,
            &["bundleId", "version", "locale", "displayType", "files"],
        ),
        (
            "testflight_status",
            true,
            false,
            true,
            &["bundleId", "buildNumber"],
        ),
        (
            "testflight_distribute",
            false,
            false,
            true,
            &["bundleId", "buildNumber", "groupNames"],
        ),
        (
            "testflight_remove",
            false,
            true,
            true,
            &["bundleId", "buildNumber", "groupNames"],
        ),
        (
            "review_submit",
            false,
            false,
            false,
            &["bundleId", "version"],
        ),
        ("release", false, true, false, &["bundleId", "version"]),
    ];
    assert_eq!(specs.len(), expected.len());
    for &(name, read_only, destructive, idempotent, required) in expected {
        let spec = specs
            .iter()
            .find(|s| s.name == name)
            .unwrap_or_else(|| panic!("missing {name}"));
        assert_eq!(spec.hints.read_only, Some(read_only), "{name}");
        assert_eq!(spec.hints.destructive, Some(destructive), "{name}");
        assert_eq!(spec.hints.idempotent, Some(idempotent), "{name}");
        assert_eq!(spec.hints.open_world, Some(true), "{name}");
        assert_eq!(spec.input_schema["required"], json!(required), "{name}");
        assert_eq!(spec.input_schema["additionalProperties"], false, "{name}");
        assert_eq!(
            spec.input_schema["properties"].get("dryRun").is_some(),
            !read_only,
            "{name}"
        );
    }
    let fields = json!([
        "description",
        "keywords",
        "whatsNew",
        "promotionalText",
        "marketingUrl",
        "supportUrl"
    ]);
    let read = specs.iter().find(|s| s.name == "metadata_get").unwrap();
    assert_eq!(
        read.input_schema["properties"]["fields"]["items"]["enum"],
        fields
    );
    let apply = specs.iter().find(|s| s.name == "metadata_apply").unwrap();
    let locale = &apply.input_schema["properties"]["localizations"]["additionalProperties"];
    assert!(
        locale.get("required").is_none(),
        "omitted fields remain optional"
    );
    assert_eq!(locale["properties"].as_object().unwrap().len(), 6);
    for field in fields.as_array().unwrap() {
        assert_eq!(
            locale["properties"][field.as_str().unwrap()]["type"],
            json!(["string", "null"])
        );
    }
}
#[test]
fn advanced_profiles_preserve_raw_operations_and_composite_lookup_dependencies() {
    let all = server("all").tool_names();
    for descriptor in crate::tools::operation_catalog() {
        assert!(
            all.iter().any(|n| n == descriptor.name),
            "missing {}",
            descriptor.name
        );
    }
    for (profile, needed) in [
        (
            "release_raw",
            vec![
                "apps_getCollection",
                "appStoreVersions_updateInstance",
                "appScreenshotSets_createInstance",
            ],
        ),
        (
            "testflight_raw",
            vec![
                "apps_getCollection",
                "builds_getCollection",
                "betaGroups_builds_createToManyRelationship",
            ],
        ),
        (
            "monetization_raw",
            vec![
                "apps_getCollection",
                "subscriptions_createInstance",
                "territories_getCollection",
            ],
        ),
        (
            "signing_raw",
            vec!["apps_getCollection", "certificates_deleteInstance"],
        ),
        (
            "reports_raw",
            vec![
                "apps_getCollection",
                "salesReports_getCollection",
                "analytics_segment_download",
            ],
        ),
    ] {
        let names = server(profile).tool_names();
        for name in needed {
            assert!(names.iter().any(|n| n == name), "{profile} lacks {name}");
        }
        assert!(!names.iter().any(|n| n == "review_submit"));
    }
}
#[test]
fn generated_operations_have_expected_workflow_request_capabilities() {
    let catalog = crate::tools::operation_catalog();
    // Independent contract: method + required input capabilities relied upon by services.
    let contracts: &[(&str, &str, &[&str])] = &[
        (
            "apps_getCollection",
            "GET",
            &["filter[bundleId]", "fields[apps]", "cursor"],
        ),
        (
            "apps_appStoreVersions_getToManyRelated",
            "GET",
            &["id", "filter[versionString]", "filter[platform]"],
        ),
        (
            "builds_getCollection",
            "GET",
            &[
                "filter[app]",
                "filter[version]",
                "filter[preReleaseVersion.platform]",
                "filter[preReleaseVersion.version]",
            ],
        ),
        ("appStoreVersions_build_getToOneRelated", "GET", &["id"]),
        (
            "appStoreVersions_appStoreVersionPhasedRelease_getToOneRelated",
            "GET",
            &["id"],
        ),
        (
            "appStoreVersions_appStoreReviewDetail_getToOneRelated",
            "GET",
            &["id"],
        ),
        ("apps_appInfos_getToManyRelated", "GET", &["id"]),
        (
            "appInfos_ageRatingDeclaration_getToOneRelated",
            "GET",
            &["id"],
        ),
        (
            "appStoreVersions_appStoreVersionLocalizations_getToManyRelated",
            "GET",
            &["id", "cursor"],
        ),
        (
            "appStoreVersionLocalizations_createInstance",
            "POST",
            &["data"],
        ),
        (
            "appStoreVersionLocalizations_updateInstance",
            "PATCH",
            &["id", "data"],
        ),
        (
            "appStoreVersionLocalizations_appScreenshotSets_getToManyRelated",
            "GET",
            &["id"],
        ),
        (
            "appScreenshotSets_appScreenshots_getToManyRelated",
            "GET",
            &["id"],
        ),
        ("appScreenshotSets_createInstance", "POST", &["data"]),
        (
            "appScreenshotSets_appScreenshots_replaceToManyRelationship",
            "PATCH",
            &["id", "data"],
        ),
        ("appScreenshots_deleteInstance", "DELETE", &["id"]),
        ("apps_betaGroups_getToManyRelated", "GET", &["id"]),
        ("betaGroups_builds_getToManyRelationship", "GET", &["id"]),
        (
            "betaGroups_builds_createToManyRelationship",
            "POST",
            &["id", "data"],
        ),
        (
            "betaGroups_builds_deleteToManyRelationship",
            "DELETE",
            &["id", "data"],
        ),
        ("builds_buildBetaDetail_getToOneRelated", "GET", &["id"]),
        (
            "reviewSubmissions_getCollection",
            "GET",
            &["filter[app]", "filter[platform]"],
        ),
        (
            "reviewSubmissions_items_getToManyRelated",
            "GET",
            &["id", "include"],
        ),
        ("reviewSubmissions_createInstance", "POST", &["data"]),
        ("reviewSubmissionItems_createInstance", "POST", &["data"]),
        ("reviewSubmissions_updateInstance", "PATCH", &["id", "data"]),
        (
            "appStoreVersionReleaseRequests_createInstance",
            "POST",
            &["data"],
        ),
        (
            "analyticsReports_instances_getToManyRelated",
            "GET",
            &["id"],
        ),
        (
            "analyticsReportInstances_segments_getToManyRelated",
            "GET",
            &["id"],
        ),
        ("analyticsReportSegments_getInstance", "GET", &["id"]),
        (
            "salesReports_getCollection",
            "GET",
            &[
                "filter[vendorNumber]",
                "filter[reportDate]",
                "filter[frequency]",
                "filter[reportType]",
                "filter[reportSubType]",
            ],
        ),
        (
            "financeReports_getCollection",
            "GET",
            &[
                "filter[vendorNumber]",
                "filter[reportDate]",
                "filter[regionCode]",
                "filter[reportType]",
            ],
        ),
    ];
    for &(name, method, fields) in contracts {
        let descriptor = catalog
            .iter()
            .find(|op| op.name == name)
            .unwrap_or_else(|| panic!("workflow dependency missing: {name}"));
        let spec = (descriptor.build)();
        let ExecutionKind::Rest(rest) = &spec.execution else {
            panic!("{name} must remain REST")
        };
        assert_eq!(rest.method, method, "{name}");
        assert_eq!(
            spec.hints.read_only,
            Some(method == "GET"),
            "{name} safety classification changed"
        );
        for field in fields {
            assert!(
                spec.input_schema["properties"].get(*field).is_some(),
                "{name} lost input {field}"
            );
        }
    }
    for kind in super::asset_upload::KINDS {
        for name in [kind.get_tool, kind.create_tool, kind.update_tool] {
            assert!(
                catalog.iter().any(|op| op.name == name),
                "upload dependency {name} missing"
            );
        }
    }
}
