//! Release inspection and reconciliation. Apple remains the submission validator.
use super::{
    failure,
    screenshot_inventory::{complete, ScreenshotInventory},
    services::{compact, Asc},
};
use mcp_factory_core::ToolResult;
use serde_json::{json, Value};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReleaseAction {
    VersionGet,
    Status,
    Prepare,
    Submit,
    Release,
}

fn editable_for_submission(state: &str) -> bool {
    matches!(
        state,
        "PREPARE_FOR_SUBMISSION" | "DEVELOPER_REJECTED" | "REJECTED" | "METADATA_REJECTED"
    )
}

pub(super) async fn execute(
    client: &Asc<'_>,
    action: ReleaseAction,
    args: &Value,
) -> Result<Value, ToolResult> {
    let (app, version) = client.version(args).await?;
    match action {
        ReleaseAction::VersionGet => {
            let build = attached_build(client, &version).await?;
            Ok(
                json!({"app":compact(&app,&["name","bundleId"]),"version":compact(&version,&["versionString","platform","appStoreState","releaseType","earliestReleaseDate"]),"build":compact_optional(&build,&["version","processingState","expired"])}),
            )
        }
        ReleaseAction::Status => status(client, &app, &version).await,
        ReleaseAction::Prepare => preflight(client, args, &app, &version).await,
        ReleaseAction::Submit => submit(client, args, &app, &version).await,
        ReleaseAction::Release => release(client, args, &version).await,
    }
}
fn compact_optional(resource: &Value, fields: &[&str]) -> Value {
    if resource.is_null() {
        Value::Null
    } else {
        compact(resource, fields)
    }
}
async fn attached_build(client: &Asc<'_>, version: &Value) -> Result<Value, ToolResult> {
    Ok(client.optional("appStoreVersions_build_getToOneRelated",json!({"id":version["id"],"fields[builds]":["version","processingState","expired","expirationDate","usesNonExemptEncryption"]})).await?["data"].clone())
}
async fn submissions(
    client: &Asc<'_>,
    app: &Value,
    version: &Value,
) -> Result<Vec<(Value, Vec<Value>)>, ToolResult> {
    let rows=client.pages("reviewSubmissions_getCollection",json!({"filter[app]":[app["id"]],"filter[platform]":[version["attributes"]["platform"]],"fields[reviewSubmissions]":["state","platform","appStoreVersionForReview"],"limit":200})).await?;
    if rows.len() > 50 {
        return Err(failure(
            "lookup",
            "too many review submissions to reconcile automatically; use release_raw",
            Value::Null,
        ));
    }
    let mut results = Vec::new();
    for row in rows {
        let items=client.pages("reviewSubmissions_items_getToManyRelated",json!({"id":row["id"],"include":["appStoreVersion"],"fields[reviewSubmissionItems]":["state","appStoreVersion"],"fields[appStoreVersions]":["versionString","platform"],"limit":200})).await?;
        results.push((row, items));
    }
    Ok(results)
}
fn has_version(items: &[Value], version: &Value) -> bool {
    items
        .iter()
        .any(|item| item["relationships"]["appStoreVersion"]["data"]["id"] == version["id"])
}
async fn status(client: &Asc<'_>, app: &Value, version: &Value) -> Result<Value, ToolResult> {
    let build = attached_build(client, version).await?;
    let phased=client.optional("appStoreVersions_appStoreVersionPhasedRelease_getToOneRelated",json!({"id":version["id"],"fields[appStoreVersionPhasedReleases]":["phasedReleaseState","currentDayNumber","totalPauseDuration","startDate"]})).await?;
    let submissions = submissions(client, app, version).await?;
    let relevant: Vec<_> = submissions
        .iter()
        .filter(|(_, items)| has_version(items, version))
        .map(|(row, _)| compact(row, &["state", "platform"]))
        .collect();
    let mut issues = Vec::new();
    if build.is_null() {
        issues.push("No build attached");
    } else if build["attributes"]["processingState"] != "VALID" {
        issues.push("Attached build is not VALID");
    }
    if build["attributes"]["expired"] == true {
        issues.push("Attached build has expired");
    }
    Ok(
        json!({"app":compact(app,&["name","bundleId"]),"version":compact(version,&["versionString","platform","appStoreState","releaseType","earliestReleaseDate"]),"build":compact_optional(&build,&["version","processingState","expired","expirationDate"]),"reviewSubmissions":relevant.iter().take(5).collect::<Vec<_>>(),"reviewSubmissionCount":relevant.len(),"phasedRelease":compact_optional(&phased["data"],&["phasedReleaseState","currentDayNumber","totalPauseDuration","startDate"]),"blockingIssues":issues,"warnings":[]}),
    )
}
async fn preflight(
    client: &Asc<'_>,
    args: &Value,
    app: &Value,
    version: &Value,
) -> Result<Value, ToolResult> {
    let mut blockers = Vec::<String>::new();
    let mut warnings = Vec::<String>::new();
    let mut checks = json!({"app":"ok","version":"ok"});
    let state = version["attributes"]["appStoreState"]
        .as_str()
        .unwrap_or("UNKNOWN");
    if !editable_for_submission(state) {
        blockers.push(format!(
            "Version state {state} is not editable for submission"
        ));
        checks["version"] = json!("blocked");
    }
    let build = attached_build(client, version).await?;
    if build.is_null() {
        blockers.push("No build attached".into());
        checks["build"] = json!("missing");
    } else {
        checks["build"] = compact(&build, &["version", "processingState", "expired"]);
        if build["attributes"]["processingState"] != "VALID" {
            blockers.push("Build processing state is not VALID".into());
        }
        if build["attributes"]["expired"] == true {
            blockers.push("Build has expired".into());
        }
        match build["attributes"]["usesNonExemptEncryption"].as_bool() {
            None => {
                blockers.push("Build export-compliance answer is missing".into());
                checks["exportCompliance"] = json!("missing");
            }
            Some(false) => checks["exportCompliance"] = json!("answer_recorded"),
            Some(true) => {
                checks["exportCompliance"] = json!("requires_apple_evaluation");
                warnings.push(
                    "Non-exempt encryption declaration/approval requires Apple evaluation".into(),
                );
            }
        }
    }
    let locales = client
        .localizations(version, None, &["locale", "description", "supportUrl"])
        .await?;
    let primary = app["attributes"]["primaryLocale"]
        .as_str()
        .unwrap_or("en-US");
    let primary_rows: Vec<_> = locales
        .iter()
        .filter(|r| r["attributes"]["locale"] == primary)
        .collect();
    if primary_rows.len() != 1 {
        blockers.push(format!(
            "Primary localization {primary} is missing or ambiguous"
        ));
        checks["metadata"] = json!("missing");
    } else {
        let loc = primary_rows[0];
        checks["metadata"] = json!("checked_primary_locale");
        for field in ["description", "supportUrl"] {
            if loc["attributes"][field]
                .as_str()
                .is_none_or(|s| s.trim().is_empty())
            {
                blockers.push(format!("Primary localization {field} is missing"));
            }
        }
        let inventory = ScreenshotInventory { client };
        let sets = inventory.sets(loc).await?;
        let required: Vec<_> = args["requiredDisplayTypes"]
            .as_array()
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        let mut complete_types = Vec::new();
        for set in sets {
            let shots = inventory.assets(&set, &["assetDeliveryState"]).await?;
            let display = set["attributes"]["screenshotDisplayType"]
                .as_str()
                .unwrap_or("UNKNOWN")
                .to_owned();
            if complete(&shots) {
                complete_types.push(display);
            } else {
                warnings.push(format!(
                    "Screenshot set {display} is empty or still processing"
                ));
            }
        }
        if complete_types.is_empty() {
            blockers.push("No complete screenshots in primary locale".into());
        }
        for display in required {
            if !complete_types.iter().any(|t| t == display) {
                blockers.push(format!(
                    "Required screenshot display type {display} is missing or incomplete"
                ));
            }
        }
        checks["screenshots"] = json!({"completeDisplayTypes":complete_types,"requirementCoverage":if args["requiredDisplayTypes"].is_array(){"caller_specified"}else{"unknown"}});
    }
    let details=client.optional("appStoreVersions_appStoreReviewDetail_getToOneRelated",json!({"id":version["id"],"fields[appStoreReviewDetails]":["contactFirstName","contactLastName","contactPhone","contactEmail","demoAccountRequired","demoAccountName","demoAccountPassword"]})).await?;
    checks["reviewDetails"] = json!("checked");
    if details["data"].is_null() {
        blockers.push("App Review details are missing".into());
        checks["reviewDetails"] = json!("missing");
    } else {
        for field in [
            "contactFirstName",
            "contactLastName",
            "contactEmail",
            "contactPhone",
        ] {
            if details["data"]["attributes"][field]
                .as_str()
                .is_none_or(|s| s.trim().is_empty())
            {
                blockers.push(format!("App Review {field} is missing"));
            }
        }
        if details["data"]["attributes"]["demoAccountRequired"] == true {
            for field in ["demoAccountName", "demoAccountPassword"] {
                if details["data"]["attributes"][field]
                    .as_str()
                    .is_none_or(|s| s.is_empty())
                {
                    blockers.push(format!("Required App Review {field} is missing"));
                }
            }
        }
    }
    let infos = client
        .pages(
            "apps_appInfos_getToManyRelated",
            json!({"id":app["id"],"fields[appInfos]":["appStoreState"],"limit":200}),
        )
        .await?;
    let editable: Vec<_> = infos
        .iter()
        .filter(|info| info["attributes"]["appStoreState"] == state)
        .collect();
    if editable.len() == 1 {
        let age = client
            .optional(
                "appInfos_ageRatingDeclaration_getToOneRelated",
                json!({"id":editable[0]["id"]}),
            )
            .await?;
        checks["ageRating"] = json!(if age["data"].is_null() {
            "missing"
        } else {
            "declaration_present"
        });
        if age["data"].is_null() {
            blockers.push("Age-rating declaration is missing".into());
        }
    } else {
        checks["ageRating"] = json!("unknown");
        warnings.push(
            "Editable App Info could not be selected uniquely; age rating remains unknown".into(),
        );
    }
    let unknown = json!([
        "Apple submission validation",
        "App Privacy labels (web UI)",
        "Monetization review/compliance",
        "Required device screenshot coverage unless explicitly supplied",
        "Completeness of age-rating answers"
    ]);
    let blocker_count = blockers.len();
    let warning_count = warnings.len();
    Ok(
        json!({"app":compact(app,&["name","bundleId"]),"version":compact(version,&["versionString","platform","appStoreState"]),"checks":checks,"readyForSubmission":blocker_count==0,"readiness":if blocker_count==0{"no_known_blockers"}else{"blocked"},"blockingIssues":blockers.into_iter().take(50).collect::<Vec<_>>(),"blockingIssueCount":blocker_count,"warnings":warnings.into_iter().take(30).collect::<Vec<_>>(),"warningCount":warning_count,"unknownChecks":unknown}),
    )
}
async fn submit(
    client: &Asc<'_>,
    args: &Value,
    app: &Value,
    version: &Value,
) -> Result<Value, ToolResult> {
    let rows = submissions(client, app, version).await?;
    let editable = editable_for_submission(
        version["attributes"]["appStoreState"]
            .as_str()
            .unwrap_or(""),
    );
    let matching: Vec<_> = rows
        .iter()
        .filter(|(submission, items)| {
            has_version(items, version)
                && !(editable && submission["attributes"]["state"] == "COMPLETE")
        })
        .collect();
    if matching.len() > 1 {
        return Err(failure(
            "reconcile",
            "multiple submissions reference this version; use release_raw",
            Value::Null,
        ));
    }
    if let Some((submission, _)) = matching.first() {
        let state = submission["attributes"]["state"]
            .as_str()
            .unwrap_or("UNKNOWN");
        if ["WAITING_FOR_REVIEW", "IN_REVIEW", "COMPLETING", "COMPLETE"].contains(&state) {
            return Ok(
                json!({"changes":[],"unchanged":1,"submission":compact(submission,&["state"]),"warnings":[],"dryRun":args["dryRun"]==true}),
            );
        }
        if state != "READY_FOR_REVIEW" {
            return Err(failure(
                "reconcile",
                "existing submission is not usable; resolve its issues in release_raw",
                compact(submission, &["state"]),
            ));
        }
    }
    if rows.iter().any(|(s, items)| {
        !has_version(items, version)
            && [
                "WAITING_FOR_REVIEW",
                "IN_REVIEW",
                "UNRESOLVED_ISSUES",
                "CANCELING",
                "COMPLETING",
            ]
            .contains(&s["attributes"]["state"].as_str().unwrap_or(""))
    }) {
        return Err(failure(
            "reconcile",
            "another submission is active for this app/platform",
            Value::Null,
        ));
    }
    if rows.iter().any(|(submission, items)| {
        let state = submission["attributes"]["state"]
            .as_str()
            .unwrap_or("UNKNOWN");
        (state == "READY_FOR_REVIEW" && !items.is_empty() && !has_version(items, version))
            || ![
                "READY_FOR_REVIEW",
                "WAITING_FOR_REVIEW",
                "IN_REVIEW",
                "UNRESOLVED_ISSUES",
                "CANCELING",
                "COMPLETING",
                "COMPLETE",
            ]
            .contains(&state)
    }) {
        return Err(failure(
            "reconcile",
            "existing draft contains unrelated items or has unknown state; use release_raw",
            Value::Null,
        ));
    }
    let check = preflight(client, args, app, version).await?;
    if check["readyForSubmission"] != true {
        return Err(failure("preflight", "known submission blockers", check));
    }
    let candidates: Vec<_> = rows
        .iter()
        .filter(|(s, items)| {
            s["attributes"]["state"] == "READY_FOR_REVIEW"
                && (has_version(items, version) || items.is_empty())
        })
        .collect();
    if candidates.len() > 1 {
        return Err(failure(
            "reconcile",
            "multiple usable draft submissions; use release_raw",
            Value::Null,
        ));
    }
    if let Some((_, items)) = candidates.first() {
        if items
            .iter()
            .any(|i| i["relationships"]["appStoreVersion"]["data"]["id"] != version["id"])
        {
            return Err(failure(
                "reconcile",
                "draft includes unrelated review items; refusing to submit them",
                Value::Null,
            ));
        }
    }
    let create = candidates.is_empty();
    let add_item = create || !has_version(&candidates[0].1, version);
    let mut changes = Vec::new();
    if create {
        changes.push(json!({"action":"create","resource":"reviewSubmission"}));
    }
    if add_item {
        changes.push(
            json!({"action":"create","resource":"reviewSubmissionItem","versionId":version["id"]}),
        );
    }
    changes.push(json!({"action":"submit","resource":"reviewSubmission"}));
    if args["dryRun"] == true {
        return Ok(
            json!({"changes":changes,"warnings":check["warnings"],"unknownChecks":check["unknownChecks"],"dryRun":true}),
        );
    }
    let submission = if create {
        client.write("reviewSubmissions_createInstance",json!({"data":{"type":"reviewSubmissions","attributes":{"platform":version["attributes"]["platform"]},"relationships":{"app":{"data":{"type":"apps","id":app["id"]}}}}})).await?["data"].clone()
    } else {
        candidates[0].0.clone()
    };
    if submission["id"].as_str().is_none() {
        return Err(failure(
            "apply",
            "submission response missing ID",
            Value::Null,
        ));
    }
    let id = &submission["id"];
    let apply=async {
        if add_item {client.write("reviewSubmissionItems_createInstance",json!({"data":{"type":"reviewSubmissionItems","relationships":{"reviewSubmission":{"data":{"type":"reviewSubmissions","id":id}},"appStoreVersion":{"data":{"type":"appStoreVersions","id":version["id"]}}}}})).await?;}
        client.write("reviewSubmissions_updateInstance",json!({"id":id,"data":{"type":"reviewSubmissions","id":id,"attributes":{"submitted":true}}})).await
    }.await;
    match apply {
        Ok(result) => Ok(
            json!({"changes":changes,"submission":compact(&result["data"],&["state"]),"warnings":check["warnings"],"unknownChecks":check["unknownChecks"],"dryRun":false}),
        ),
        Err(error) => Err(failure(
            "apply",
            "submission interrupted; retry reconciles the existing draft",
            json!({"reviewSubmissionId":id,"versionId":version["id"],"upstream":error.structured}),
        )),
    }
}
async fn release(client: &Asc<'_>, args: &Value, version: &Value) -> Result<Value, ToolResult> {
    let state = version["attributes"]["appStoreState"]
        .as_str()
        .unwrap_or("UNKNOWN");
    if [
        "READY_FOR_DISTRIBUTION",
        "READY_FOR_SALE",
        "PROCESSING_FOR_DISTRIBUTION",
        "PROCESSING_FOR_APP_STORE",
    ]
    .contains(&state)
    {
        return Ok(
            json!({"changes":[],"unchanged":1,"state":state,"warnings":[],"dryRun":args["dryRun"]==true}),
        );
    }
    if state != "PENDING_DEVELOPER_RELEASE" {
        return Err(failure(
            "preflight",
            "manual release requires PENDING_DEVELOPER_RELEASE",
            json!({"state":state}),
        ));
    }
    let changes =
        json!([{"action":"release","resource":"appStoreVersion","versionId":version["id"]}]);
    if args["dryRun"] == true {
        return Ok(json!({"changes":changes,"dryRun":true,"warnings":[]}));
    }
    let result=client.write("appStoreVersionReleaseRequests_createInstance",json!({"data":{"type":"appStoreVersionReleaseRequests","relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":version["id"]}}}}})).await?;
    Ok(
        json!({"changes":changes,"releaseRequestId":result["data"]["id"],"state":"release_requested","dryRun":false,"warnings":[]}),
    )
}

#[cfg(test)]
mod action_tests {
    #[test]
    fn submission_editability_accepts_rejected_states_and_refuses_unknown_or_released_states() {
        for state in [
            "PREPARE_FOR_SUBMISSION",
            "DEVELOPER_REJECTED",
            "REJECTED",
            "METADATA_REJECTED",
        ] {
            assert!(super::editable_for_submission(state), "{state}");
        }
        for state in [
            "",
            "UNKNOWN",
            "FUTURE_APPLE_STATE",
            "WAITING_FOR_REVIEW",
            "PENDING_DEVELOPER_RELEASE",
            "READY_FOR_DISTRIBUTION",
        ] {
            assert!(!super::editable_for_submission(state), "{state}");
        }
    }
}
