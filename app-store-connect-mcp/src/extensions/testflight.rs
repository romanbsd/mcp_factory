//! Build-to-group TestFlight workflows. Validate all targets before mutation.
use super::{
    contracts::{value, ChangeReport, GroupChange},
    failure,
    output_budget::json_bytes,
    progress::ApplyProgress,
    services::{compact, required, unique, Asc},
};
use mcp_factory_core::ToolResult;
use serde_json::{json, Value};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum TestflightAction {
    Status,
    Distribute,
    Remove,
}

pub(super) async fn execute(
    client: &Asc<'_>,
    action: TestflightAction,
    args: &Value,
) -> Result<Value, ToolResult> {
    let status = action == TestflightAction::Status;
    let offset = if status {
        match args.get("offset") {
            Some(value) => value
                .as_u64()
                .and_then(|n| usize::try_from(n).ok())
                .ok_or_else(|| {
                    failure(
                        "validate",
                        "offset must be a nonnegative integer",
                        Value::Null,
                    )
                })?,
            None => 0,
        }
    } else {
        0
    };
    let names = match args.get("groupNames") {
        Some(value) => Some(
            value
                .as_array()
                .filter(|a| status || !a.is_empty())
                .ok_or_else(|| {
                    failure(
                        "validate",
                        "groupNames must be a nonempty array",
                        Value::Null,
                    )
                })?
                .iter()
                .map(|v| {
                    v.as_str()
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .ok_or_else(|| {
                            failure(
                                "validate",
                                "groupNames must contain nonempty strings",
                                Value::Null,
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        None if status => None,
        None => return Err(failure("validate", "groupNames is required", Value::Null)),
    };
    let (app, build) = client.build(args).await?;
    let query = json!({"id":app["id"],"fields[betaGroups]":["name","isInternalGroup","hasAccessToAllBuilds"],"limit":200});
    let (selected, group_count, next_offset, observed) = if status && names.is_none() {
        let window = client
            .window("apps_betaGroups_getToManyRelated", query, offset, 25)
            .await?;
        let next = window
            .has_more
            .then_some(offset.saturating_add(window.rows.len()));
        (window.rows, window.total, next, window.observed)
    } else {
        let groups = client
            .pages("apps_betaGroups_getToManyRelated", query)
            .await?;
        let mut selected = Vec::new();
        if let Some(names) = names {
            let mut seen = std::collections::HashSet::new();
            for name in names {
                if seen.insert(name.clone()) {
                    selected.push(unique(
                        groups
                            .iter()
                            .filter(|g| g["attributes"]["name"] == name)
                            .cloned()
                            .collect(),
                        &format!("beta group {name}"),
                    )?);
                }
            }
        } else {
            selected = groups;
        }
        let count = selected.len();
        let next = if status && offset.saturating_add(25) < count {
            Some(offset + 25)
        } else {
            None
        };
        let selected = if status {
            selected.into_iter().skip(offset).take(25).collect()
        } else {
            selected
        };
        (selected, Some(count), next, count)
    };
    let mut membership = Vec::new();
    // Read every selected group before writing, so lookup failures cannot cause partial changes.
    for group in selected {
        let rows = client
            .pages(
                "betaGroups_builds_getToManyRelationship",
                json!({"id":group["id"],"limit":200}),
            )
            .await?;
        let assigned = rows.iter().any(|r| r["id"] == build["id"]);
        membership.push((group, assigned));
    }
    if status {
        let details=client.get("builds_buildBetaDetail_getToOneRelated",json!({"id":build["id"],"fields[buildBetaDetails]":["internalBuildState","externalBuildState","autoNotifyEnabled"]})).await?;
        let scanned_groups = membership.len();
        let mut report = json!({"build":compact(&build,&["version","processingState","expired","expirationDate"]),"betaDetails":compact(&details["data"],&["internalBuildState","externalBuildState","autoNotifyEnabled"]),"groups":membership.into_iter().filter(|(g,assigned)|*assigned || g["attributes"]["hasAccessToAllBuilds"]==true).map(|(g,assigned)|json!({"id":g["id"],"name":g["attributes"]["name"],"isInternalGroup":g["attributes"]["isInternalGroup"],"assigned":assigned,"hasAccessToAllBuilds":g["attributes"]["hasAccessToAllBuilds"]})).collect::<Vec<_>>(),"groupCount":group_count,"scannedGroups":scanned_groups,"nextOffset":next_offset,"warnings":[]});
        if group_count.is_none() {
            report["groupCountLowerBound"] = json!(observed);
        }
        if json_bytes(&report) > 10_000 {
            return Err(failure(
                "lookup",
                "TestFlight status exceeds 10KB; select fewer groupNames",
                Value::Null,
            ));
        }
        return Ok(report);
    }
    let remove = action == TestflightAction::Remove;
    let dry_run = args["dryRun"].as_bool().unwrap_or(false);
    let mut changes = Vec::new();
    let mut unchanged = Vec::new();
    let mut warnings = Vec::new();
    let mut pending = Vec::new();
    for (group, assigned) in membership {
        let all = group["attributes"]["hasAccessToAllBuilds"] == true;
        if remove && all {
            warnings.push(json!({"groupName":group["attributes"]["name"],"message":"group has access to all builds; removing an explicit relationship does not revoke that access"}));
        }
        if (remove && !assigned) || (!remove && (assigned || all)) {
            unchanged.push(required(&group["attributes"], "name")?.to_owned());
            continue;
        }
        changes.push(GroupChange {
            action: if remove { "remove" } else { "assign" },
            resource: "betaGroupBuild",
            group_name: required(&group["attributes"], "name")?.to_owned(),
            build_id: required(&build, "id")?.to_owned(),
        });
        pending.push(json!({"id":group["id"],"data":[{"type":"builds","id":build["id"]}]}));
    }
    if !dry_run {
        let mut progress = ApplyProgress::new(value(&changes).as_array().unwrap().clone());
        for request in pending {
            let result = client
                .write(
                    if remove {
                        "betaGroups_builds_deleteToManyRelationship"
                    } else {
                        "betaGroups_builds_createToManyRelationship"
                    },
                    request,
                )
                .await;
            match result {
                Ok(_) => progress.applied(),
                Err(error) => return Err(progress.interrupted(error)),
            }
        }
    }
    Ok(value(ChangeReport {
        changes,
        unchanged,
        warnings,
        dry_run,
    }))
}
