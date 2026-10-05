//! `reports_installs_query`: reads only the monthly `app_version` exports that
//! intersect the requested dates, then filters and aggregates with explicit
//! semantics. Absence is never turned into zero.

use std::collections::{BTreeMap, BTreeSet};

use mcp_factory_core::chrono::{Datelike, NaiveDate, Utc};
use serde_json::{json, Value};

use super::parse::{self, Aggregation, InstallRow, Metric};
use super::resolve::{Failure, INSTALLS_PREFIX};
use super::storage::StorageError;
use super::{failure, package_arg, report_id, setup, InstallsContext, Recorder, MAX_PARSE_BYTES};

const MAX_DAYS: i64 = 92;
const MAX_ROWS: usize = 1000;
const MAX_ISSUES_PER_SOURCE: usize = 50;

struct Request {
    package: String,
    start: NaiveDate,
    end: NaiveDate,
    versions: Option<BTreeSet<u64>>,
    metrics: Option<Vec<&'static Metric>>,
    include_rows: bool,
}

fn request(arguments: &Value) -> Result<Request, Failure> {
    let invalid = |message: &str| Failure::new("invalid_request", message);
    let package = package_arg(arguments)?;
    let date = |key: &str| {
        arguments[key]
            .as_str()
            .and_then(|text| NaiveDate::parse_from_str(text, "%Y-%m-%d").ok())
            .ok_or_else(|| invalid(&format!("{key} must be an ISO date (YYYY-MM-DD)")))
    };
    let (start, end) = (date("startDate")?, date("endDate")?);
    if end < start {
        return Err(invalid("endDate is before startDate"));
    }
    if (end - start).num_days() >= MAX_DAYS {
        return Err(invalid(&format!(
            "the date range is limited to {MAX_DAYS} days"
        )));
    }
    if arguments["dimension"].as_str().unwrap_or("app_version") != "app_version" {
        return Err(invalid("only the app_version breakdown is supported"));
    }
    let versions = match arguments["dimensionValues"].as_array() {
        Some(values) => Some(
            values
                .iter()
                .map(|value| match value {
                    Value::String(text) => parse::version_code(text),
                    other => other.as_u64(),
                })
                .collect::<Option<BTreeSet<u64>>>()
                .ok_or_else(|| invalid("dimensionValues must be version codes"))?,
        ),
        None => None,
    };
    let metrics = match arguments["metrics"].as_array() {
        Some(keys) => Some(
            keys.iter()
                .map(|key| key.as_str().and_then(parse::metric_by_key))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| invalid("unknown metric"))?,
        ),
        None => None,
    };
    Ok(Request {
        package,
        start,
        end,
        versions,
        metrics,
        include_rows: arguments["includeRows"].as_bool().unwrap_or(true),
    })
}

pub(super) fn months(start: NaiveDate, end: NaiveDate) -> Vec<(i32, u32)> {
    let mut months = Vec::new();
    let (mut year, mut month) = (start.year(), start.month());
    while (year, month) <= (end.year(), end.month()) {
        months.push((year, month));
        (year, month) = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };
    }
    months
}

struct SourceRows {
    report_id: String,
    rows: Vec<InstallRow>,
    metrics: Vec<&'static Metric>,
}

pub async fn run(ctx: &InstallsContext, recorder: &Recorder, arguments: &Value) -> Value {
    let request = match request(arguments) {
        Ok(request) => request,
        Err(error) => return failure(error),
    };
    let (storage, account, resolution, candidates) =
        match setup(ctx, recorder, arguments, &request.package).await {
            Ok(setup) => setup,
            Err(envelope) => return envelope,
        };
    let bucket = &resolution.bucket;
    // One entry per intersecting month that exists or failed; its `state`
    // says how far it got, so failed months are derived rather than tracked.
    let mut sources = Vec::new();
    let mut parsed = Vec::new();
    let mut missing_months = Vec::new();
    let mut warnings = Vec::new();
    for (year, month) in months(request.start, request.end) {
        let label = format!("{year}-{month:02}");
        let object = format!(
            "{INSTALLS_PREFIX}{}_{year}{month:02}_app_version.csv",
            request.package
        );
        let meta = match recorder.metadata(storage.as_ref(), bucket, &object).await {
            Ok(meta) => meta,
            Err(StorageError::ObjectNotFound(_)) => {
                missing_months.push(label);
                continue;
            }
            Err(error) => {
                ctx.evict(&account, storage.as_ref(), &request.package, &error);
                sources.push(json!({"month": label, "objectName": object, "state": "metadata_failed", "error": error.to_json()}));
                continue;
            }
        };
        let mut source = super::descriptor(bucket, &meta);
        let bytes = match ctx
            .download(storage.as_ref(), recorder, bucket, &meta, MAX_PARSE_BYTES)
            .await
        {
            Ok(bytes) => bytes,
            Err(error) => {
                source["state"] = json!("download_failed");
                source["error"] = error.to_json();
                sources.push(source);
                continue;
            }
        };
        let outcome = parse::decode(&bytes)
            .map_err(|message| parse::SchemaError {
                message,
                headers: Vec::new(),
            })
            .and_then(|(text, encoding)| {
                source["encoding"] = json!(encoding);
                parse::parse_installs(&text, &request.package, request.metrics.as_deref())
            });
        match outcome {
            Ok(installs) => {
                source["state"] = json!("parsed");
                source["headers"] = json!(installs.headers);
                if !installs.issues.is_empty() {
                    source["rejectedRows"] = json!(installs.issues.len());
                    source["issues"] = json!(installs
                        .issues
                        .iter()
                        .take(MAX_ISSUES_PER_SOURCE)
                        .collect::<Vec<_>>());
                    warnings.push(json!({
                        "reportId": source["reportId"],
                        "message": format!("{} malformed or conflicting rows were rejected, not counted", installs.issues.len())
                    }));
                }
                parsed.push(SourceRows {
                    report_id: report_id(bucket, &meta),
                    rows: installs.rows,
                    metrics: installs.metrics,
                });
            }
            Err(error) => {
                source["state"] = json!("schema_error");
                source["error"] = json!({"code": "schema_mismatch", "message": error.message, "headers": error.headers});
                warnings.push(json!({
                    "reportId": source["reportId"],
                    "message": "Export could not be interpreted; its original bytes remain available via reports_installs_get_raw."
                }));
            }
        }
        sources.push(source);
    }
    let failed_months: Vec<Value> = sources
        .iter()
        .filter(|source| source["state"] != "parsed")
        .map(|source| json!({"month": source["month"], "error": source["error"]}))
        .collect();

    let in_range: Vec<(&str, &InstallRow)> = parsed
        .iter()
        .flat_map(|source| {
            source
                .rows
                .iter()
                .filter(|row| row.date >= request.start && row.date <= request.end)
                .map(|row| (source.report_id.as_str(), row))
        })
        .collect();
    let observed_dates: BTreeSet<NaiveDate> = in_range.iter().map(|(_, row)| row.date).collect();
    let mut matched: Vec<(&str, &InstallRow)> = in_range
        .into_iter()
        .filter(|(_, row)| {
            request
                .versions
                .as_ref()
                .is_none_or(|versions| versions.contains(&row.version))
        })
        .collect();
    matched.sort_by_key(|(_, row)| (row.date, row.version));
    // Union of metrics mapped by any source, in table order.
    let metrics: Vec<&'static Metric> = parse::METRICS
        .iter()
        .filter(|metric| {
            parsed
                .iter()
                .any(|source| source.metrics.iter().any(|seen| seen.key == metric.key))
        })
        .collect();

    let mut by_version: BTreeMap<u64, Vec<&InstallRow>> = BTreeMap::new();
    for (_, row) in &matched {
        by_version.entry(row.version).or_default().push(row);
    }
    let summaries: Vec<Value> = by_version
        .iter()
        .map(|(version, rows)| summarize(*version, rows, &metrics))
        .collect();
    let requested_not_observed: Vec<String> = request
        .versions
        .iter()
        .flatten()
        .filter(|version| !by_version.contains_key(version))
        .map(u64::to_string)
        .collect();
    let dates_without: Vec<String> = request
        .start
        .iter_days()
        .take_while(|date| *date <= request.end)
        .filter(|date| !observed_dates.contains(date))
        .map(|date| date.to_string())
        .collect();
    let rows: Vec<Value> = matched
        .iter()
        .take(if request.include_rows { MAX_ROWS } else { 0 })
        .map(|(report, row)| {
            json!({
                "date": row.date.to_string(),
                "dimensionValue": row.version.to_string(),
                "values": row.values,
                "reportId": report,
                "sourceRowNumber": row.row_number,
            })
        })
        .collect();

    let all_failed = !failed_months.is_empty() && parsed.is_empty() && missing_months.is_empty();
    let incomplete = !failed_months.is_empty()
        || !missing_months.is_empty()
        || !dates_without.is_empty()
        || !warnings.is_empty();
    let status = if all_failed {
        "error"
    } else if matched.is_empty() {
        if failed_months.is_empty() {
            "no_data"
        } else {
            "partial"
        }
    } else if incomplete {
        "partial"
    } else {
        "ok"
    };
    if !dates_without.is_empty() {
        warnings.push(json!({"message": format!(
            "{} requested dates have no observations; they are unreported, not zero installs",
            dates_without.len()
        )}));
    }
    if matched.is_empty() && status != "error" {
        warnings.push(json!({"message": "No matching rows: this does not mean zero installs or a healthy release."}));
    }
    let retrieved_at = Utc::now().to_rfc3339();
    json!({
        "status": status,
        "error": all_failed.then(|| json!({"code": "sources_unavailable", "message": "Every intersecting export failed to load or parse.", "failedMonths": failed_months})),
        "request": {
            "account": account.developer_id,
            "packageName": request.package,
            "startDate": request.start.to_string(),
            "endDate": request.end.to_string(),
            "dimension": "app_version",
            "dimensionValues": request.versions.as_ref().map(|versions| versions.iter().map(u64::to_string).collect::<Vec<_>>()),
            "metrics": request.metrics.as_ref().map(|metrics| metrics.iter().map(|m| m.key).collect::<Vec<_>>()),
        },
        "summaries": summaries,
        "rows": rows,
        "rowsTruncated": request.include_rows && matched.len() > MAX_ROWS,
        "matchedRowCount": matched.len(),
        "coverage": {
            "observedFrom": observed_dates.first().map(ToString::to_string),
            "observedTo": observed_dates.last().map(ToString::to_string),
            "missingMonths": missing_months,
            "failedMonths": failed_months,
            "datesWithoutObservations": dates_without,
            "requestedVersionsNotObserved": requested_not_observed,
        },
        "timeZone": {
            "status": "unknown",
            "note": "Google does not document the day boundary of install exports; do not assume a GA4 property time zone."
        },
        "freshness": {
            "latestObservedDate": observed_dates.last().map(ToString::to_string),
            "objectsUpdated": sources.iter().map(|s| json!({"reportId": s["reportId"], "updated": s["updated"]})).collect::<Vec<_>>(),
            "retrievedAt": retrieved_at,
            "note": "Observation dates are the CSV Date column; object updated time is when Google last rewrote the monthly file; retrievedAt is this read. Google posts daily data within 3-7 days and gives no update schedule, so recent dates are usually absent rather than zero."
        },
        "sources": sources,
        "resolution": resolution.to_json(),
        "discovery": {"candidates": candidates},
        "retrievedAt": retrieved_at,
        "warnings": warnings,
    })
}

fn summarize(version: u64, rows: &[&InstallRow], metrics: &[&'static Metric]) -> Value {
    let metric_summaries: Vec<Value> = metrics
        .iter()
        .map(|metric| {
            let observed: Vec<(NaiveDate, i64)> = rows
                .iter()
                .filter_map(|row| {
                    row.values
                        .get(metric.key)
                        .copied()
                        .flatten()
                        .map(|v| (row.date, v))
                })
                .collect();
            let blank_cells = rows
                .iter()
                .filter(|row| matches!(row.values.get(metric.key), Some(None)))
                .count();
            let mut summary = json!({
                "metric": metric.key,
                "sourceColumn": metric.column,
                "observedDates": observed.len(),
                "blankCells": blank_cells,
            });
            match metric.aggregation {
                Aggregation::Event => {
                    summary["aggregation"] = json!("sum_over_observed_dates");
                    summary["value"] = json!((!observed.is_empty())
                        .then(|| observed.iter().map(|(_, v)| v).sum::<i64>()));
                    if metric.key.starts_with("dailyUser") {
                        summary["note"] = json!(
                            "Sum of daily user counts; not a distinct-user count for the interval."
                        );
                    }
                }
                Aggregation::Snapshot => {
                    let latest = observed.iter().max_by_key(|(date, _)| *date);
                    summary["aggregation"] = json!("latest_snapshot");
                    summary["value"] = json!(latest.map(|(_, v)| v));
                    summary["asOfDate"] = json!(latest.map(|(date, _)| date.to_string()));
                }
            }
            summary
        })
        .collect();
    json!({
        "dimensionValue": version.to_string(),
        "firstObservedDate": rows.iter().map(|row| row.date).min().map(|d| d.to_string()),
        "lastObservedDate": rows.iter().map(|row| row.date).max().map(|d| d.to_string()),
        "metrics": metric_summaries,
    })
}
