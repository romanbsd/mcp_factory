use mcp_factory_core::chrono::{Datelike, Duration, NaiveDate, Utc};
use serde_json::{json, Value};

use super::client::EvidenceClient;
use super::registry;

const DEFAULT_METRICS: &[&str] = &[
    "crash_rate",
    "user_perceived_crash_rate",
    "anr_rate",
    "user_perceived_anr_rate",
    "user_perceived_lmk_rate",
    "slow_start_rate",
    "excessive_wakeup_rate",
    "stuck_background_wakelock_rate",
];

pub async fn report(client: &mut EvidenceClient<'_>, arguments: &Value) -> Value {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    let lookback = arguments["lookbackDays"]
        .as_i64()
        .unwrap_or(90)
        .clamp(1, 365);
    let cohorts = string_list(arguments.get("cohorts"), &["OS_PUBLIC", "APP_TESTERS"]);
    let metrics = string_list(arguments.get("metrics"), DEFAULT_METRICS);
    let mut results = Vec::new();

    for metric_name in metrics {
        let Some(capability) = registry::metric(&metric_name) else {
            results.push(unsupported_metric(&metric_name, "unknown metric"));
            continue;
        };
        for cohort in &cohorts {
            if cohort != "OS_PUBLIC" && cohort != "APP_TESTERS" {
                results.push(unsupported_metric(
                    &metric_name,
                    &format!("unsupported cohort {cohort}"),
                ));
                continue;
            }
            if cohort == "APP_TESTERS" && !capability.tester_supported {
                results.push(json!({
                    "metric": metric_name,
                    "cohort": cohort,
                    "dataState": "unsupported",
                    "evidenceStrength": "none",
                    "assessment": "This metric/cohort combination is excluded by the validated capability registry."
                }));
                continue;
            }
            results.push(query_metric(client, package, lookback, cohort, capability).await);
        }
    }

    let include_issues = arguments["includeIssues"].as_bool().unwrap_or(true);
    let include_anomalies = arguments["includeAnomalies"].as_bool().unwrap_or(true);
    let error_evidence = if include_issues {
        Some(query_error_evidence(client, package, lookback).await)
    } else {
        None
    };
    let anomaly_evidence = if include_anomalies {
        Some(query_anomalies(client, package, lookback).await)
    } else {
        None
    };

    let unavailable = results
        .iter()
        .filter(|result| result["dataState"] == "unavailable")
        .count();
    let summary = if results.is_empty() {
        "No supported quality metrics were requested.".to_string()
    } else if unavailable > 0 {
        format!(
            "Quality evidence is partial: {unavailable} metric/cohort queries were unavailable."
        )
    } else {
        "Quality evidence was normalized without treating missing rows as zero.".to_string()
    };

    json!({
        "status": client.status(),
        "generatedAt": Utc::now().to_rfc3339(),
        "app": app(package),
        "summary": summary,
        "findings": [],
        "actions": [],
        "coverageGaps": registry::console_coverage_gaps(),
        "sourceCalls": client.source_calls.clone(),
        "warnings": client.warnings.clone(),
        "metrics": results,
        "errorEvidence": error_evidence,
        "anomalyEvidence": anomaly_evidence,
        "requestedLookbackDays": lookback,
    })
}

async fn query_error_evidence(
    client: &mut EvidenceClient<'_>,
    package: &str,
    lookback: i64,
) -> Value {
    let name = format!("apps/{package}/errorCountMetricSet");
    let metadata = client
        .call(
            "call-error-counts-freshness",
            "vitals_errors_counts_get",
            json!({"name": name}),
        )
        .await;
    let end = metadata
        .as_ref()
        .and_then(daily_freshness)
        .unwrap_or_else(default_end);
    let start = subtract_days(&end, lookback).unwrap_or_else(|| default_start(lookback));
    let (count_pages, counts_complete) = client
        .call_pages(
            "call-error-counts",
            "vitals_errors_counts_query",
            json!({
                "name": name,
                "body": {
                    "timelineSpec": {"aggregationPeriod": "DAILY", "startTime": start, "endTime": end},
                    "dimensions": ["reportType"],
                    "metrics": ["errorReportCount", "distinctUsers"],
                    "pageSize": 100000
                }
            }),
            &["body", "pageToken"],
            &["nextPageToken"],
        )
        .await;
    let issue_args = interval_arguments(package, lookback);
    let (issue_pages, issues_complete) = client
        .call_pages(
            "call-error-issues",
            "vitals_errors_issues_search",
            issue_args,
            &["pageToken"],
            &["nextPageToken"],
        )
        .await;
    json!({
        "countRows": count_pages.iter().map(|page| page["rows"].as_array().map_or(0, Vec::len)).sum::<usize>(),
        "groupedIssues": issue_pages.iter().map(|page| page["errorIssues"].as_array().map_or(0, Vec::len)).sum::<usize>(),
        "countsPaginationComplete": counts_complete,
        "issuesPaginationComplete": issues_complete,
        "assessment": "Absolute error counts and grouped issues are preserved as independent evidence sources."
    })
}

async fn query_anomalies(client: &mut EvidenceClient<'_>, package: &str, lookback: i64) -> Value {
    let end = Utc::now();
    let start = end - Duration::days(lookback);
    let filter = format!(
        "activeBetween(\"{}\", \"{}\")",
        start.to_rfc3339(),
        end.to_rfc3339()
    );
    let (pages, complete) = client
        .call_pages(
            "call-anomalies",
            "anomalies_list",
            json!({"parent": format!("apps/{package}"), "pageSize": 100, "filter": filter}),
            &["pageToken"],
            &["nextPageToken"],
        )
        .await;
    json!({
        "count": pages.iter().map(|page| page["anomalies"].as_array().map_or(0, Vec::len)).sum::<usize>(),
        "paginationComplete": complete,
        "assessment": "Anomalies are a separate signal and do not prove that Console-only checks passed."
    })
}

fn interval_arguments(package: &str, lookback: i64) -> Value {
    let end = Utc::now().date_naive();
    let start = end - Duration::days(lookback);
    // Unlike the DAILY vitals timelineSpecs (which default to
    // America/Los_Angeles), errorIssues search intervals are UTC-only; the
    // Los_Angeles id is rejected here.
    json!({
        "parent": format!("apps/{package}"),
        "pageSize": 1000,
        "sampleErrorReportLimit": 0,
        "interval.startTime.year": start.year(),
        "interval.startTime.month": start.month(),
        "interval.startTime.day": start.day(),
        "interval.startTime.timeZone.id": "UTC",
        "interval.endTime.year": end.year(),
        "interval.endTime.month": end.month(),
        "interval.endTime.day": end.day(),
        "interval.endTime.timeZone.id": "UTC"
    })
}

async fn query_metric(
    client: &mut EvidenceClient<'_>,
    package: &str,
    lookback: i64,
    cohort: &str,
    capability: &registry::MetricCapability,
) -> Value {
    let resource = format!("apps/{package}/{}", capability.resource_suffix);
    let get_method = format!("{}_get", capability.method_prefix);
    let query_method = format!("{}_query", capability.method_prefix);
    debug_assert!(registry::allowed(&get_method));
    debug_assert!(registry::allowed(&query_method));

    let freshness_id = format!("call-{}-{cohort}-freshness", capability.public_name);
    let Some(metadata) = client
        .call(&freshness_id, &get_method, json!({"name": resource}))
        .await
    else {
        return unavailable_metric(
            capability.public_name,
            cohort,
            "freshness metadata unavailable",
        );
    };
    let end = daily_freshness(&metadata).unwrap_or_else(default_end);
    let start = subtract_days(&end, lookback).unwrap_or_else(|| default_start(lookback));
    let body = json!({
        "timelineSpec": {
            "aggregationPeriod": "DAILY",
            "startTime": start,
            "endTime": end,
        },
        "metrics": [capability.metric_name, "distinctUsers"],
        "dimensions": capability.required_dimensions,
        "userCohort": cohort,
    });
    let query_id = format!("call-{}-{cohort}-query", capability.public_name);
    let (pages, complete) = client
        .call_pages(
            &query_id,
            &query_method,
            json!({"name": resource, "body": body}),
            &["body", "pageToken"],
            &["nextPageToken"],
        )
        .await;
    if pages.is_empty() && !complete {
        return unavailable_metric(capability.public_name, cohort, "metric query unavailable");
    }
    let rows: Vec<&Value> = pages
        .iter()
        .flat_map(|page| page["rows"].as_array().into_iter().flatten())
        .collect();
    normalize_rows(capability, cohort, lookback, &start, &end, &rows, complete)
}

fn normalize_rows(
    capability: &registry::MetricCapability,
    cohort: &str,
    requested_days: i64,
    start: &Value,
    end: &Value,
    rows: &[&Value],
    complete: bool,
) -> Value {
    if rows.is_empty() {
        return json!({
            "metric": capability.public_name,
            "cohort": cohort,
            "dataState": "no_data",
            "observedDays": 0,
            "returnedObservationCount": 0,
            "requestedDays": requested_days,
            "effectiveWindow": {"start": start, "end": end, "endExclusive": true, "timeZone": "America/Los_Angeles"},
            "latestDataEnd": end,
            "maxDailyDistinctUsers": null,
            "pointEstimateMaximum": null,
            "confidenceIntervalUpperMaximum": null,
            "evidenceStrength": "none",
            "evidenceStrengthReason": "The source returned no observations.",
            "paginationComplete": complete,
            "assessment": "The query succeeded with no rows; this is insufficient data, not a zero rate."
        });
    }

    let mut point_max = 0.0_f64;
    let mut confidence_max = 0.0_f64;
    let mut users_max = 0.0_f64;
    for row in rows {
        for metric in row["metrics"].as_array().into_iter().flatten() {
            let name = metric["metric"].as_str().unwrap_or_default();
            if name == capability.metric_name {
                point_max = point_max.max(decimal(metric.get("decimalValue")));
                confidence_max = confidence_max.max(decimal(
                    metric
                        .get("decimalValueConfidenceInterval")
                        .and_then(|interval| interval.get("upperBound")),
                ));
            } else if name == "distinctUsers" {
                users_max = users_max.max(decimal(metric.get("decimalValue")));
            }
        }
    }
    let data_state = if point_max > 0.0 {
        "observed_nonzero"
    } else {
        "observed_zero"
    };
    // Distinct daily timeline points, not row count: metrics with a required
    // dimension (e.g. slow_start_rate/startType) return several rows per day, so
    // rows.len() would overstate coverage and evidence strength. Falls back to
    // the row count only when rows carry no startTime to key days on.
    let observed_days = match distinct_days(rows) {
        0 => rows.len(),
        days => days,
    };
    let evidence = if observed_days < 7 || users_max < 20.0 || !complete {
        "low"
    } else if observed_days >= 28 && users_max >= 100.0 && confidence_max <= point_max + 0.05 {
        "high"
    } else {
        "medium"
    };
    let evidence_reason = match evidence {
        "low" => "Fewer than 7 days, fewer than 20 maximum daily users, or incomplete pagination limits the conclusion.",
        "high" => "At least 28 days, at least 100 maximum daily users, complete pagination, and a narrow confidence interval support the conclusion.",
        _ => "The evidence is complete but exposure or uncertainty does not meet the high-strength threshold.",
    };
    let assessment = match (data_state, evidence) {
        ("observed_zero", "low") => format!(
            "No {} events were observed, but the sample is too small or incomplete for a strong health claim.",
            capability.public_name
        ),
        ("observed_zero", _) => format!(
            "No {} events were observed in the returned evidence window.",
            capability.public_name
        ),
        _ => format!(
            "At least one non-zero {} observation was returned.",
            capability.public_name
        ),
    };
    json!({
        "metric": capability.public_name,
        "cohort": cohort,
        "dataState": data_state,
        "observedDays": observed_days,
        "returnedObservationCount": rows.len(),
        "requestedDays": requested_days,
        "effectiveWindow": {"start": start, "end": end, "endExclusive": true, "timeZone": "America/Los_Angeles"},
        "latestDataEnd": end,
        "maxDailyDistinctUsers": users_max,
        "pointEstimateMaximum": point_max,
        "confidenceIntervalUpperMaximum": confidence_max,
        "evidenceStrength": evidence,
        "evidenceStrengthReason": evidence_reason,
        "paginationComplete": complete,
        "assessment": assessment,
    })
}

/// Counts distinct calendar days across rows, keyed on each row's `startTime`
/// (year, month, day). Returns 0 when any row lacks a `startTime`, letting the
/// caller fall back to the raw row count — a partially keyed set would otherwise
/// silently drop the unkeyed rows and understate coverage.
fn distinct_days(rows: &[&Value]) -> usize {
    let mut days = std::collections::BTreeSet::new();
    for row in rows {
        let time = &row["startTime"];
        if let (Some(year), Some(month), Some(day)) = (
            time["year"].as_i64(),
            time["month"].as_i64(),
            time["day"].as_i64(),
        ) {
            days.insert((year, month, day));
        } else {
            return 0;
        }
    }
    days.len()
}

fn decimal(value: Option<&Value>) -> f64 {
    value
        .and_then(|value| value.get("value"))
        .and_then(Value::as_str)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0.0)
}

fn daily_freshness(metadata: &Value) -> Option<Value> {
    let latest = metadata["freshnessInfo"]["freshnesses"]
        .as_array()?
        .iter()
        .find(|item| item["aggregationPeriod"] == "DAILY")?
        .get("latestEndTime")?;
    // DAILY aggregation requires hours/minutes/seconds/nanos to be unset, but
    // the freshness response is a full DateTime and may carry them — forward
    // only the calendar date and time zone, or the query is rejected.
    Some(json!({
        "year": latest["year"],
        "month": latest["month"],
        "day": latest["day"],
        "timeZone": latest["timeZone"],
    }))
}

fn subtract_days(end: &Value, days: i64) -> Option<Value> {
    let date = NaiveDate::from_ymd_opt(
        end["year"].as_i64()? as i32,
        end["month"].as_u64()? as u32,
        end["day"].as_u64()? as u32,
    )? - Duration::days(days);
    Some(date_value(date))
}

fn default_end() -> Value {
    date_value(Utc::now().date_naive())
}

fn default_start(days: i64) -> Value {
    date_value(Utc::now().date_naive() - Duration::days(days))
}

fn date_value(date: NaiveDate) -> Value {
    json!({
        "year": date.year(),
        "month": date.month(),
        "day": date.day(),
        "timeZone": {"id": "America/Los_Angeles"}
    })
}

fn string_list(value: Option<&Value>, defaults: &[&str]) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .filter(|values: &Vec<String>| !values.is_empty())
        .unwrap_or_else(|| defaults.iter().map(|value| (*value).to_string()).collect())
}

fn unsupported_metric(metric: &str, reason: &str) -> Value {
    json!({
        "metric": metric,
        "cohort": null,
        "dataState": "unsupported",
        "evidenceStrength": "none",
        "assessment": reason,
    })
}

fn unavailable_metric(metric: &str, cohort: &str, reason: &str) -> Value {
    json!({
        "metric": metric,
        "cohort": cohort,
        "dataState": "unavailable",
        "evidenceStrength": "none",
        "assessment": reason,
    })
}

pub fn app(package: &str) -> Value {
    json!({
        "packageName": package,
        "displayName": null,
        "resourceName": format!("apps/{package}"),
    })
}
