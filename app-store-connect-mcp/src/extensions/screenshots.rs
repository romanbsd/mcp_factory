//! Ordered screenshot reconciliation. Uploads use the internal upload service.
use super::{
    asset_upload::AssetUpload,
    contracts::{value, ScreenshotChange},
    failure,
    output_budget::{json_bytes, ArrayBudget},
    screenshot_inventory::{state, ScreenshotInventory, ASSET_FIELDS},
    services::{required, unique, Asc},
    upload_session::{FileIdentity, PreparedFile},
};
use mcp_factory_core::ToolResult;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::sync::OnceLock;

const ORDER: &str = "appScreenshotSets_appScreenshots_replaceToManyRelationship";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ScreenshotAction {
    Get,
    Sync,
}

pub(super) async fn execute(
    client: &Asc<'_>,
    action: ScreenshotAction,
    args: &Value,
    upload: &AssetUpload,
) -> Result<Value, ToolResult> {
    match action {
        ScreenshotAction::Get => get(client, args).await,
        ScreenshotAction::Sync => sync(client, args, upload).await,
    }
}
async fn localizations(client: &Asc<'_>, args: &Value) -> Result<Vec<Value>, ToolResult> {
    let (_, version) = client.version(args).await?;
    client.localizations(&version, None, &["locale"]).await
}
async fn sets(client: &Asc<'_>, localization: &Value) -> Result<Vec<Value>, ToolResult> {
    ScreenshotInventory { client }.sets(localization).await
}
async fn assets(client: &Asc<'_>, set: &Value) -> Result<Vec<Value>, ToolResult> {
    ScreenshotInventory { client }
        .assets(set, ASSET_FIELDS)
        .await
}
fn normalized(row: &Value, position: usize) -> Value {
    json!({"id":row["id"],"fileName":row["attributes"]["fileName"],"fileSize":row["attributes"]["fileSize"],
        "state":state(row),"checksum":row["attributes"]["sourceFileChecksum"],"position":position})
}
async fn get(client: &Asc<'_>, args: &Value) -> Result<Value, ToolResult> {
    let locales = localizations(client, args).await?;
    let selected = args["locales"].as_array();
    let mut seen = HashSet::new();
    let mut locales_to_scan = Vec::new();
    for locale in locales {
        let name = required(&locale["attributes"], "locale")?;
        if selected.is_some_and(|xs| !xs.iter().any(|x| x.as_str() == Some(name))) {
            continue;
        }
        if !seen.insert(name.to_owned()) {
            return Err(failure("lookup", "ambiguous localization", json!(name)));
        }
        locales_to_scan.push(locale);
    }
    if let Some(selected) = selected {
        for requested in selected {
            if !seen.contains(requested.as_str().unwrap_or_default()) {
                return Err(failure(
                    "lookup",
                    "requested localization not found",
                    requested.clone(),
                ));
            }
        }
    }
    let offset = args["offset"]
        .as_u64()
        .and_then(|n| usize::try_from(n).ok())
        .unwrap_or(0);
    let mut result = json!({"ok":true,"screenshots":[],"hasMore":false,"nextOffset":null});
    // Reserve 32 bytes for the continuation offset and hasMore envelope change.
    let mut budget = ArrayBudget::new(19_500 - json_bytes(&result) + 2 - 32);
    let mut index = 0usize;
    let inventory = ScreenshotInventory { client };
    for locale in locales_to_scan {
        let name = required(&locale["attributes"], "locale")?;
        for set in inventory.sets(&locale).await? {
            let display = &set["attributes"]["screenshotDisplayType"];
            if args
                .get("displayType")
                .is_some_and(|wanted| wanted != display)
            {
                continue;
            }
            let mut pages = inventory.asset_pages(&set, ASSET_FIELDS);
            let mut position = 0usize;
            let mut asset_ids = HashSet::new();
            while let Some(page) = pages.next().await? {
                let empty = page.rows.is_empty() && position == 0 && !page.has_more;
                let header = empty.then(|| {
                    Ok(json!({"locale":name,"displayType":display,"setId":set["id"],"assets":[]}))
                });
                let rows = page.rows.into_iter().map(|row| {
                    if !asset_ids.insert(required(&row, "id")?.to_owned()) {
                        return Err(failure("lookup", "duplicate screenshot id", Value::Null));
                    }
                    let item = json!({"locale":name,"displayType":display,"setId":set["id"],"assets":[normalized(&row,position)]});
                    position += 1;
                    Ok(item)
                });
                for item in header.into_iter().chain(rows) {
                    let item = item?;
                    if index >= offset {
                        if !budget.push(&item) {
                            if index == offset {
                                return Err(failure(
                                    "lookup",
                                    "screenshot metadata exceeds output limit",
                                    Value::Null,
                                ));
                            }
                            result["hasMore"] = json!(true);
                            result["nextOffset"] = json!(index);
                            return Ok(result);
                        }
                        result["screenshots"].as_array_mut().unwrap().push(item);
                    }
                    index += 1;
                }
            }
        }
    }
    Ok(result)
}

struct File {
    identity: FileIdentity,
    matched: Option<String>,
    resume: Option<Value>,
}
fn matches(row: &Value, file: &File) -> bool {
    state(row) == "COMPLETE"
        && row["attributes"]["fileName"].as_str() == Some(&file.identity.file_name)
        && row["attributes"]["fileSize"].as_u64() == Some(file.identity.file_size)
        && row["attributes"]["sourceFileChecksum"]
            .as_str()
            .is_some_and(|s| s.eq_ignore_ascii_case(&file.identity.checksum))
}
fn change(
    resource: &str,
    action: &str,
    locale: &str,
    display: &str,
    file: Option<&str>,
    id: Option<&str>,
) -> Value {
    value(ScreenshotChange {
        resource,
        action,
        locale,
        display_type: display,
        file,
        id,
    })
}
fn fail_workflow(error: ToolResult, result: &Value) -> ToolResult {
    let upstream = error
        .structured
        .clone()
        .unwrap_or_else(|| json!({"message":error.into_text()}));
    failure(
        "apply",
        "screenshot reconciliation stopped; completed uploads can be resumed",
        json!({"result":result,"upstream":upstream}),
    )
}
async fn sync(client: &Asc<'_>, args: &Value, upload: &AssetUpload) -> Result<Value, ToolResult> {
    let locale = required(args, "locale")?;
    let display = required(args, "displayType")?;
    static DISPLAY_TYPES: OnceLock<Option<Vec<Value>>> = OnceLock::new();
    let valid_display = DISPLAY_TYPES
        .get_or_init(|| {
            let descriptor = crate::tools::operation_catalog()
                .iter()
                .find(|op| op.name == "appScreenshotSets_createInstance")?;
            let schema = (descriptor.build)().input_schema;
            schema["properties"]["data"]["properties"]["attributes"]["properties"]
                ["screenshotDisplayType"]["enum"]
                .as_array()
                .cloned()
        })
        .as_ref()
        .is_some_and(|values| values.iter().any(|value| value.as_str() == Some(display)));
    if !valid_display {
        return Err(failure(
            "validate",
            "unsupported screenshot display type",
            json!(display),
        ));
    }
    let paths = args["files"]
        .as_array()
        .ok_or_else(|| failure("validate", "files must be an array", Value::Null))?;
    if paths.len() > 10 {
        return Err(failure(
            "validate",
            "a screenshot set supports at most 10 desired files",
            Value::Null,
        ));
    }
    let dry = args["dryRun"].as_bool().unwrap_or(false);
    let remove = args["removeObsolete"].as_bool().unwrap_or(false);
    if args.get("resumes").is_some_and(|r| !r.is_object()) {
        return Err(failure(
            "validate",
            "resumes must map file paths to checkpoints",
            Value::Null,
        ));
    }
    let validation_permit = upload
        .transfers
        .clone()
        .acquire_owned()
        .await
        .map_err(|error| failure("validate", &error.to_string(), Value::Null))?;
    let mut files = Vec::new();
    let mut seen = HashSet::new();
    let mut names = HashSet::new();
    // Resolve and hash every file before any mutation, using bounded streaming.
    for value in paths {
        let relative = value.as_str().filter(|s| !s.is_empty()).ok_or_else(|| {
            failure(
                "validate",
                "files must contain nonempty relative paths",
                Value::Null,
            )
        })?;
        if !seen.insert(relative.to_string()) {
            return Err(failure("validate", "duplicate file path", json!(relative)));
        }
        let prepared = PreparedFile::inspect(upload.media_root.as_deref(), relative, None).await?;
        if !names.insert(prepared.identity.file_name.clone()) {
            return Err(failure(
                "validate",
                "duplicate screenshot file name",
                json!(prepared.identity.file_name),
            ));
        }
        files.push(File {
            identity: prepared.identity,
            matched: None,
            resume: args["resumes"].get(relative).cloned(),
        });
    }
    if args["resumes"]
        .as_object()
        .is_some_and(|r| r.keys().any(|key| !seen.contains(key)))
    {
        return Err(failure(
            "validate",
            "resume checkpoint refers to a file outside the desired list",
            Value::Null,
        ));
    }
    let localization = unique(
        localizations(client, args)
            .await?
            .into_iter()
            .filter(|r| r["attributes"]["locale"] == locale)
            .collect(),
        "localization",
    )?;
    let mut set = sets(client, &localization)
        .await?
        .into_iter()
        .find(|s| s["attributes"]["screenshotDisplayType"] == display);
    let existing = if let Some(set) = &set {
        assets(client, set).await?
    } else {
        Vec::new()
    };
    let mut desired_ids = HashSet::new();
    for file in &mut files {
        let candidates: Vec<_> = existing.iter().filter(|r| matches(r, file)).collect();
        if candidates.len() > 1 {
            return Err(failure(
                "lookup",
                "ambiguous identical existing screenshots",
                json!(file.identity.file),
            ));
        }
        file.matched = candidates
            .first()
            .map(|r| r["id"].as_str().unwrap().to_string());
        if let Some(id) = &file.matched {
            desired_ids.insert(id.clone());
        }
        if let Some(cp) = &file.resume {
            let parent = set.as_ref().and_then(|s| s["id"].as_str()).ok_or_else(|| {
                failure(
                    "validate",
                    "resume requires an existing screenshot set",
                    json!(file.identity.file),
                )
            })?;
            let inspection = upload
                .inspect_resume(client.invoker, "appScreenshot", parent, &file.identity, cp)
                .await?;
            if !existing
                .iter()
                .any(|r| r["id"] == inspection.resource["id"])
            {
                return Err(failure(
                    "validate",
                    "resume asset identity or screenshot set membership does not match",
                    json!(file.identity.file),
                ));
            }
            if file
                .matched
                .as_ref()
                .is_some_and(|id| cp["assetId"] != id.as_str())
            {
                return Err(failure(
                    "validate",
                    "resume conflicts with a matching complete screenshot",
                    json!(file.identity.file),
                ));
            }
            desired_ids.insert(cp["assetId"].as_str().unwrap().to_string());
        }
    }
    let obsolete: Vec<_> = existing
        .iter()
        .filter(|r| !desired_ids.contains(r["id"].as_str().unwrap()))
        .collect();
    if !remove && files.len() + obsolete.len() > 10 {
        return Err(failure("validate","desired and retained screenshots exceed set capacity; explicitly select removeObsolete to replace assets",Value::Null));
    }
    // UploadService acquires its own permit; release preflight's budget before mutation.
    drop(validation_permit);
    let mut result = json!({"ok":true,"dryRun":dry,"changes":[],"unchanged":[],"warnings":[],"pending":[],"resumes":{}});
    for file in &files {
        if let Some(id) = &file.matched {
            result["unchanged"].as_array_mut().unwrap().push(change(
                "screenshot",
                "unchanged",
                locale,
                display,
                Some(&file.identity.file),
                Some(id),
            ));
        }
    }
    if !remove && !obsolete.is_empty() {
        result["warnings"]
            .as_array_mut()
            .unwrap()
            .push(json!(format!(
                "Retaining {} existing screenshots after the desired files",
                obsolete.len()
            )));
    }
    if set.is_none() && !files.is_empty() {
        result["changes"]
            .as_array_mut()
            .unwrap()
            .push(change("set", "create", locale, display, None, None));
        if !dry {
            let created=client.write("appScreenshotSets_createInstance",json!({"data":{"type":"appScreenshotSets","attributes":{"screenshotDisplayType":display},"relationships":{"appStoreVersionLocalization":{"data":{"type":"appStoreVersionLocalizations","id":localization["id"]}}}}})).await.map_err(|e|fail_workflow(e,&result))?;
            required(&created["data"], "id")?;
            set = Some(created["data"].clone());
        }
    }
    let mut ordered = Vec::new();
    let mut pending = false;
    for file in &files {
        if let Some(id) = &file.matched {
            ordered.push(id.clone());
            continue;
        }
        result["changes"].as_array_mut().unwrap().push(change(
            "screenshot",
            if file.resume.is_some() {
                "resume"
            } else {
                "upload"
            },
            locale,
            display,
            Some(&file.identity.file),
            None,
        ));
        if dry {
            ordered.push(
                file.resume
                    .as_ref()
                    .and_then(|r| r["assetId"].as_str())
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("planned:{}", file.identity.file)),
            );
            continue;
        }
        let mut upload_args = json!({"kind":"appScreenshot","parentId":set.as_ref().unwrap()["id"],"file":file.identity.file,"waitSeconds":args["waitSeconds"].as_u64().unwrap_or(120)});
        if let Some(cp) = &file.resume {
            upload_args["resume"] = cp.clone();
        }
        let body = match upload.run(client.invoker, &upload_args).await {
            Ok(r) => r.structured.unwrap_or(Value::Null),
            Err(e) => {
                if let Some(cp) = e.structured.as_ref().and_then(|b| b.get("resume")) {
                    result["resumes"][&file.identity.file] = cp.clone();
                }
                result["pending"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!({"file":file.identity.file,"state":"FAILED"}));
                return Err(fail_workflow(e, &result));
            }
        };
        result["resumes"][&file.identity.file] = body["resume"].clone();
        let id = required(&body["asset"], "assetId")?;
        ordered.push(id.to_string());
        if body["asset"]["state"] != "COMPLETE" {
            pending = true;
            result["pending"]
                .as_array_mut()
                .unwrap()
                .push(json!({"file":file.identity.file,"id":id,"state":body["asset"]["state"]}));
        }
    }
    if pending {
        result["warnings"].as_array_mut().unwrap().push(json!(
            "Ordering and removal deferred until every desired screenshot is COMPLETE"
        ));
        return Ok(result);
    }
    if !remove {
        ordered.extend(
            obsolete
                .iter()
                .map(|r| r["id"].as_str().unwrap().to_string()),
        );
    }
    let current: Vec<_> = existing
        .iter()
        .map(|r| r["id"].as_str().unwrap().to_string())
        .collect();
    if ordered != current {
        result["changes"].as_array_mut().unwrap().push(json!({"resource":"order","action":"replace","locale":locale,"displayType":display,"ids":ordered}));
        if !dry {
            client.write(ORDER,json!({"id":set.as_ref().unwrap()["id"],"data":ordered.iter().map(|id|json!({"type":"appScreenshots","id":id})).collect::<Vec<_>>()})).await.map_err(|e|fail_workflow(e,&result))?;
        }
    }
    if remove {
        for row in obsolete {
            let id = row["id"].as_str().unwrap();
            result["changes"].as_array_mut().unwrap().push(change(
                "screenshot",
                "delete",
                locale,
                display,
                None,
                Some(id),
            ));
            if !dry {
                client
                    .write("appScreenshots_deleteInstance", json!({"id":id}))
                    .await
                    .map_err(|e| fail_workflow(e, &result))?;
            }
        }
    }
    Ok(result)
}
