//! Version localization reads and exact, repeatable diffs.
use super::{
    contracts::{value, ChangeReport, MetadataChange, METADATA_FIELDS as FIELDS},
    failure,
    output_budget::json_bytes,
    progress::ApplyProgress,
    services::{compact, Asc},
};
use mcp_factory_core::ToolResult;
use serde_json::{json, Map, Value};

const OUTPUT_LIMIT: usize = 20_000;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum MetadataAction {
    Get,
    Apply,
}

pub(super) async fn execute(
    client: &Asc<'_>,
    action: MetadataAction,
    args: &Value,
) -> Result<Value, ToolResult> {
    let apply = action == MetadataAction::Apply;
    let desired = if apply {
        let map = args["localizations"]
            .as_object()
            .filter(|m| !m.is_empty())
            .ok_or_else(|| {
                failure(
                    "validate",
                    "localizations must be a nonempty locale-to-fields object",
                    Value::Null,
                )
            })?;
        for (locale, fields) in map {
            if locale.is_empty() {
                return Err(failure("validate", "locale must not be empty", Value::Null));
            }
            let fields = fields.as_object().ok_or_else(|| {
                failure("validate", "locale fields must be an object", json!(locale))
            })?;
            for (field, value) in fields {
                if !FIELDS.contains(&field.as_str()) || !(value.is_string() || value.is_null()) {
                    return Err(failure(
                        "validate",
                        "unsupported localization field or value",
                        json!({"locale":locale,"field":field}),
                    ));
                }
            }
        }
        Some(map)
    } else {
        None
    };
    let (app, version) = client.version(args).await?;
    let locales: Vec<String> = if let Some(map) = desired {
        map.keys().cloned().collect()
    } else if let Some(selected) = args.get("locales") {
        let selected = selected
            .as_array()
            .filter(|a| !a.is_empty())
            .ok_or_else(|| failure("validate", "locales must be a nonempty array", Value::Null))?;
        selected
            .iter()
            .map(|v| {
                v.as_str()
                    .filter(|s| !s.is_empty())
                    .map(str::to_owned)
                    .ok_or_else(|| {
                        failure(
                            "validate",
                            "locales must contain nonempty strings",
                            Value::Null,
                        )
                    })
            })
            .collect::<Result<_, _>>()?
    } else {
        vec![app["attributes"]["primaryLocale"]
            .as_str()
            .unwrap_or("en-US")
            .to_owned()]
    };
    let fields: Vec<&str> = if !apply && args.get("fields").is_some() {
        args["fields"]
            .as_array()
            .ok_or_else(|| failure("validate", "fields must be an array", Value::Null))?
            .iter()
            .map(|v| {
                v.as_str()
                    .filter(|f| FIELDS.contains(f))
                    .ok_or_else(|| failure("validate", "unsupported metadata field", v.clone()))
            })
            .collect::<Result<_, _>>()?
    } else {
        FIELDS.to_vec()
    };
    let mut query_fields = fields.clone();
    query_fields.push("locale");
    let rows = client
        .localizations(&version, Some(&locales), &query_fields)
        .await?;
    let mut existing = std::collections::BTreeMap::new();
    for row in rows {
        let locale = row["attributes"]["locale"]
            .as_str()
            .ok_or_else(|| failure("lookup", "localization has no locale", Value::Null))?
            .to_owned();
        if locales.contains(&locale) && existing.insert(locale.clone(), row).is_some() {
            return Err(failure(
                "lookup",
                "ambiguous localization locale",
                json!(locale),
            ));
        }
    }
    if !apply {
        let result = json!({"app":compact(&app,&["bundleId","name"]),"version":compact(&version,&["versionString","platform"]),"localizations":existing.values().map(|r|compact(r,&query_fields)).collect::<Vec<_>>()});
        return bounded(result);
    }
    let dry_run = args["dryRun"].as_bool().unwrap_or(false);
    let mut changes = Vec::new();
    let mut pending = Vec::new();
    let mut unchanged = Vec::new();
    for (locale, attributes) in desired.unwrap() {
        let current = existing.get(locale);
        let diff: Map<String, Value> = attributes
            .as_object()
            .unwrap()
            .iter()
            .filter(|(key, value)| {
                current.is_none_or(|r| r["attributes"].get(*key) != Some(*value))
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if diff.is_empty() {
            unchanged.push(locale.clone());
            continue;
        }
        let action = if current.is_some() {
            "update"
        } else {
            "create"
        };
        changes.push(MetadataChange {
            action,
            resource: "localization",
            locale: locale.clone(),
            fields: diff.keys().cloned().collect(),
        });
        if let Some(row) = current {
            pending.push(("appStoreVersionLocalizations_updateInstance",json!({"id":row["id"],"data":{"type":"appStoreVersionLocalizations","id":row["id"],"attributes":diff}})));
        } else {
            let mut attrs = diff;
            attrs.insert("locale".into(), json!(locale));
            pending.push(("appStoreVersionLocalizations_createInstance",json!({"data":{"type":"appStoreVersionLocalizations","attributes":attrs,"relationships":{"appStoreVersion":{"data":{"type":"appStoreVersions","id":version["id"]}}}}})));
        }
    }
    let report = bounded(value(ChangeReport {
        changes,
        unchanged,
        warnings: vec![],
        dry_run,
    }))?;
    if !dry_run {
        let mut progress = ApplyProgress::new(
            report["changes"]
                .as_array()
                .expect("metadata report changes are an array")
                .clone(),
        );
        for (operation, request) in pending {
            if let Err(error) = client.write(operation, request).await {
                return Err(progress.interrupted(error));
            }
            progress.applied();
        }
    }
    Ok(report)
}

fn bounded(value: Value) -> Result<Value, ToolResult> {
    if json_bytes(&value) > OUTPUT_LIMIT {
        Err(failure(
            "lookup",
            "metadata output exceeds 20KB; select fewer locales or fields",
            Value::Null,
        ))
    } else {
        Ok(value)
    }
}
