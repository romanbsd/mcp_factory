use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use mcp_factory_core::{async_trait, ProxyError, ReadOnlyToolInvoker, ToolBody, ToolResult};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::parse::{self, decode, parse_installs, records};
use super::resolve::{normalize_bucket, ReportsConfig};
use super::storage::{GcsStorage, ListPage, ObjectMeta, Offsets, Storage, StorageError};
use super::{enabled, install_object, parse_report_id, tools, InstallsContext, StorageSource};

const PKG: &str = "org.example.app";
const HEADER: &str = "Date,Package Name,App Version Code,Daily Device Installs,Daily Device Upgrades,Current Device Installs,Total User Installs";

// ---------- fakes ----------

type Objects = Vec<(String, String, Vec<u8>)>; // name, generation, bytes

#[derive(Default)]
struct FakeStorage {
    buckets: Mutex<HashMap<String, Result<Objects, StorageError>>>,
    transient_failures: Mutex<u32>,
    /// Call-description prefix (e.g. `"download "`) -> error for that call.
    call_failures: Mutex<Vec<(String, StorageError)>>,
    /// Reported stored size, as for a gzip-encoded export whose stored
    /// (compressed) size is far below the delivered size.
    stored_size: Mutex<Option<u64>>,
    calls: Mutex<Vec<String>>,
}

impl FakeStorage {
    fn bucket(self, name: &str, objects: Vec<(String, &str, Vec<u8>)>) -> Self {
        let objects = objects
            .into_iter()
            .map(|(object, generation, bytes)| (object, generation.to_string(), bytes))
            .collect();
        self.buckets
            .lock()
            .unwrap()
            .insert(name.to_string(), Ok(objects));
        self
    }

    fn failing(self, name: &str, error: StorageError) -> Self {
        self.buckets
            .lock()
            .unwrap()
            .insert(name.to_string(), Err(error));
        self
    }

    fn fail_call(self, prefix: &str, error: StorageError) -> Self {
        self.call_failures
            .lock()
            .unwrap()
            .push((prefix.to_string(), error));
        self
    }

    fn calls(&self) -> Vec<String> {
        self.calls.lock().unwrap().clone()
    }

    fn objects(&self, bucket: &str, call: String) -> Result<Objects, StorageError> {
        let injected = self
            .call_failures
            .lock()
            .unwrap()
            .iter()
            .find(|(prefix, _)| call.starts_with(prefix.as_str()))
            .map(|(_, error)| error.clone());
        self.calls.lock().unwrap().push(call);
        if let Some(error) = injected {
            return Err(error);
        }
        let mut transient = self.transient_failures.lock().unwrap();
        if *transient > 0 {
            *transient -= 1;
            return Err(StorageError::Transient("HTTP 503: backend".into()));
        }
        self.buckets
            .lock()
            .unwrap()
            .get(bucket)
            .cloned()
            .unwrap_or_else(|| {
                Err(StorageError::BucketNotFound(
                    "HTTP 404: The specified bucket does not exist.".into(),
                ))
            })
    }
}

fn meta(name: &str, generation: &str, bytes: &[u8]) -> ObjectMeta {
    ObjectMeta {
        name: name.to_string(),
        generation: generation.to_string(),
        size: bytes.len() as u64,
        updated: Some("2026-10-03T08:00:00Z".to_string()),
        md5_hash: Some("md5==".to_string()),
        crc32c: Some("crc==".to_string()),
        content_encoding: None,
    }
}

#[async_trait]
impl Storage for FakeStorage {
    fn principal(&self) -> &str {
        "reports@example.iam.gserviceaccount.com"
    }

    async fn list(
        &self,
        bucket: &str,
        prefix: &str,
        (start, end): Offsets<'_>,
        page_token: Option<&str>,
        max: u32,
    ) -> Result<ListPage, StorageError> {
        let objects = self.objects(bucket, format!("list {bucket} {prefix}"))?;
        let matching: Vec<_> = objects
            .iter()
            .filter(|(name, _, _)| name.starts_with(prefix))
            .filter(|(name, _, _)| start.is_none_or(|start| name.as_str() >= start))
            .filter(|(name, _, _)| end.is_none_or(|end| name.as_str() < end))
            .collect();
        let start: usize = page_token.map_or(0, |token| token.parse().unwrap());
        let end = (start + max as usize).min(matching.len());
        Ok(ListPage {
            items: matching[start..end]
                .iter()
                .map(|(n, g, b)| meta(n, g, b))
                .collect(),
            next_page_token: (end < matching.len()).then(|| end.to_string()),
        })
    }

    async fn metadata(&self, bucket: &str, object: &str) -> Result<ObjectMeta, StorageError> {
        let objects = self.objects(bucket, format!("metadata {object}"))?;
        let stored_size = *self.stored_size.lock().unwrap();
        objects
            .iter()
            .find(|(name, _, _)| name == object)
            .map(|(n, g, b)| ObjectMeta {
                size: stored_size.unwrap_or(b.len() as u64),
                ..meta(n, g, b)
            })
            .ok_or_else(|| StorageError::ObjectNotFound("HTTP 404: No such object".into()))
    }

    async fn download(
        &self,
        bucket: &str,
        object: &str,
        generation: &str,
        max: u64,
    ) -> Result<Vec<u8>, StorageError> {
        let objects = self.objects(bucket, format!("download {object}#{generation}"))?;
        let bytes = objects
            .iter()
            .find(|(name, g, _)| name == object && g == generation)
            .map(|(_, _, bytes)| bytes.clone())
            .ok_or_else(|| StorageError::ObjectNotFound("HTTP 404: No such object".into()))?;
        if bytes.len() as u64 > max {
            return Err(StorageError::TooLarge {
                size: bytes.len() as u64,
                limit: max,
            });
        }
        Ok(bytes)
    }
}

struct NoInvoker;

#[async_trait]
impl ReadOnlyToolInvoker for NoInvoker {
    async fn invoke_read_only(&self, name: &str, _: Value) -> Result<ToolResult, ProxyError> {
        Err(ProxyError::Other(format!(
            "install tools must not call {name}"
        )))
    }
}

fn context(config: ReportsConfig, storage: Arc<FakeStorage>) -> Arc<InstallsContext> {
    Arc::new(InstallsContext::new(config, StorageSource::Ready(storage)))
}

fn dev(id: &str) -> ReportsConfig {
    ReportsConfig::from_vars(None, Some(id), None)
}

async fn call(ctx: &Arc<InstallsContext>, tool: &str, arguments: Value) -> ToolResult {
    let spec = tools(ctx.clone())
        .into_iter()
        .find(|spec| spec.name == tool)
        .unwrap();
    spec.handler.call(&NoInvoker, arguments).await.unwrap()
}

async fn run(ctx: &Arc<InstallsContext>, tool: &str, arguments: Value) -> Value {
    call(ctx, tool, arguments).await.structured.unwrap()
}

const ACCESS: &str = "reports_install_access_check";

/// Access check for `PKG` on account `123` against `storage`.
async fn probe(storage: impl Into<Arc<FakeStorage>>) -> Value {
    run(
        &context(dev("123"), storage.into()),
        ACCESS,
        json!({"packageName": PKG}),
    )
    .await
}

// ---------- fixtures ----------

fn object(month: &str) -> String {
    format!("stats/installs/installs_{PKG}_{month}_app_version.csv")
}

fn utf16le(text: &str) -> Vec<u8> {
    let mut bytes = vec![0xFF, 0xFE];
    bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
    bytes
}

fn csv(rows: &[&str]) -> String {
    let mut text = HEADER.to_string();
    for row in rows {
        text.push('\n');
        text.push_str(row);
    }
    text.push('\n');
    text
}

fn row(date: &str, version: &str, cells: &str) -> String {
    format!("{date},{PKG},{version},{cells}")
}

fn september() -> Vec<u8> {
    utf16le(&csv(&[
        &row("2026-09-29", "25", "10,1,500,900"),
        &row("2026-09-30", "25", "8,0,505,905"),
    ]))
}

fn october() -> Vec<u8> {
    utf16le(&csv(&[
        &row("2026-10-01", "25", "7,0,510,910"),
        &row("2026-10-01", "26", "3,40,43,45"),
        &row("2026-10-02", "26", "0,,48,46"),
    ]))
}

fn play_bucket() -> FakeStorage {
    FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![
            (object("202609"), "11", september()),
            (object("202610"), "21", october()),
        ],
    )
}

// ---------- decoding and parsing ----------

#[test]
fn decodes_utf16_and_utf8_variants() {
    let text = "Date,Package Name\n";
    let mut be = vec![0xFE, 0xFF];
    be.extend(text.encode_utf16().flat_map(u16::to_be_bytes));
    let mut utf8_bom = vec![0xEF, 0xBB, 0xBF];
    utf8_bom.extend(text.as_bytes());
    let no_bom: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    assert_eq!(
        decode(&utf16le(text)).unwrap(),
        (text.to_string(), "UTF-16LE (BOM)")
    );
    assert_eq!(decode(&be).unwrap(), (text.to_string(), "UTF-16BE (BOM)"));
    assert_eq!(
        decode(&utf8_bom).unwrap(),
        (text.to_string(), "UTF-8 (BOM)")
    );
    assert_eq!(
        decode(text.as_bytes()).unwrap(),
        (text.to_string(), "UTF-8")
    );
    assert_eq!(
        decode(&no_bom).unwrap(),
        (text.to_string(), "UTF-16LE (no BOM)")
    );
    assert!(decode(&[0xFF, 0xFE, 0x41]).is_err());
    assert!(decode(&[0xC3, 0x28, 0x41]).is_err());
}

#[test]
fn csv_handles_quotes_and_embedded_newlines() {
    let rows = records("a,b\n\"x, y\",\"line1\nline2\"\n\"he said \"\"hi\"\"\",\n").unwrap();
    assert_eq!(rows[1], vec!["x, y", "line1\nline2"]);
    assert_eq!(rows[2], vec!["he said \"hi\"", ""]);
}

#[test]
fn maps_by_header_name_with_reordered_and_extra_columns() {
    let text = format!(
        "Extra,Daily Device Installs,App Version Code,Date,Package Name\nz,5,26,2026-10-01,{PKG}\n"
    );
    let parsed = parse_installs(&text, PKG, None).unwrap();
    assert_eq!(
        parsed.metrics.iter().map(|m| m.key).collect::<Vec<_>>(),
        vec!["dailyDeviceInstalls"]
    );
    assert_eq!(parsed.rows[0].version, 26);
    assert_eq!(parsed.rows[0].values["dailyDeviceInstalls"], Some(5));
}

#[test]
fn missing_requested_column_is_schema_error_with_headers() {
    let requested = [parse::metric_by_key("dailyUserInstalls").unwrap()];
    let error = parse_installs(&csv(&[]), PKG, Some(&requested)).unwrap_err();
    assert!(error.message.contains("Daily User Installs"));
    assert_eq!(error.headers[0], "Date");
}

#[test]
fn rejects_invalid_rows_and_keeps_blanks_as_null() {
    let text = csv(&[
        &row("2026-10-01", "26", "1,,3,4"),
        &row("2026-10-02", "26", "x,1,3,4"),
        "2026-10-03,other.app,26,1,1,1,1",
        &row("10/04/2026", "26", "1,1,1,1"),
        &row("2026-10-05", "26", "1,1"),
        &row("2026-10-06", "26", "1,1,1,1"),
        &row("2026-10-06", "26", "2,2,2,2"),
    ]);
    let parsed = parse_installs(&text, PKG, None).unwrap();
    assert_eq!(
        parsed.rows.len(),
        1,
        "only the first row is valid and unique"
    );
    assert_eq!(parsed.rows[0].values["dailyDeviceUpgrades"], None);
    let reasons = parsed
        .issues
        .iter()
        .map(|issue| issue["reason"].as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    assert_eq!(reasons.len(), 5);
    for expected in [
        "invalid integer",
        "package mismatch",
        "invalid date",
        "cells",
        "duplicate row key",
    ] {
        assert!(
            reasons.iter().any(|reason| reason.contains(expected)),
            "{expected}: {reasons:?}"
        );
    }
}

// ---------- configuration and validation ----------

#[test]
fn bucket_validation_accepts_play_uris_and_rejects_everything_else() {
    assert_eq!(
        normalize_bucket("gs://pubsite_prod_123/").unwrap(),
        "pubsite_prod_123"
    );
    assert_eq!(
        normalize_bucket("pubsite_prod_rev_123").unwrap(),
        "pubsite_prod_rev_123"
    );
    for bad in [
        "gs://pubsite_prod_123/stats/installs",
        "https://storage.googleapis.com/pubsite_prod_123",
        "gs://user:pw@pubsite_prod_123",
        "gs://pubsite_prod_123?x=1",
        "gs://my-other-bucket",
        "gs://pubsite_prod_../x",
        "PUBSITE_PROD_1",
    ] {
        assert_eq!(
            normalize_bucket(bad).unwrap_err().code,
            "invalid_bucket",
            "{bad}"
        );
    }
}

#[test]
fn account_configuration_and_selection() {
    let config = ReportsConfig::from_vars(Some("111=gs://pubsite_prod_111,222,abc"), None, None);
    assert_eq!(config.accounts.len(), 2);
    assert_eq!(config.problems.len(), 1);
    assert_eq!(
        config.select(Some("111")).unwrap().bucket.as_deref(),
        Some("pubsite_prod_111")
    );
    assert_eq!(config.select(Some("222")).unwrap().bucket, None);
    assert_eq!(config.select(None).unwrap_err().code, "account_required");
    assert_eq!(
        config.select(Some("333")).unwrap_err().code,
        "unknown_account"
    );
    assert_eq!(
        ReportsConfig::default().select(None).unwrap_err().code,
        "missing_developer_id"
    );
}

#[test]
fn install_object_names_parse_and_reject_other_families() {
    assert_eq!(
        install_object("stats/installs/installs_com.a_202601_b_202602_os_version.csv").unwrap(),
        (
            "com.a_202601_b".to_string(),
            "2026-02".to_string(),
            "os_version".to_string()
        )
    );
    for bad in [
        "earnings/earnings_202601_123.zip",
        "stats/installs/installs_../x_202601_app_version.csv",
        "stats/crashes/crashes_com.a_202601_overview.csv",
        "stats/installs/installs_com.a_2026_app_version.csv",
    ] {
        assert!(install_object(bad).is_none(), "{bad}");
    }
}

#[test]
fn module_can_be_disabled() {
    assert!(enabled(None));
    assert!(enabled(Some("on")));
    assert!(!enabled(Some("OFF")));
    assert!(!enabled(Some("false")));
}

// ---------- discovery ----------

#[tokio::test]
async fn discovers_pubsite_prod_and_caches_mapping() {
    let storage = Arc::new(play_bucket());
    let ctx = context(dev("123"), storage.clone());
    let report = run(&ctx, ACCESS, json!({"packageName": PKG})).await;
    assert_eq!(report["status"], "ok");
    assert_eq!(report["resolution"]["provenance"], "probe:pubsite_prod");
    let states: Vec<_> = report["discovery"]["candidates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["state"].clone())
        .collect();
    assert_eq!(
        states,
        vec![json!("relevant_reports"), json!("bucket_not_found")]
    );

    let before = storage.calls().len();
    let listed = run(&ctx, "reports_installs_list", json!({"packageName": PKG})).await;
    assert_eq!(
        listed["resolution"]["provenance"],
        "cache(probe:pubsite_prod)"
    );
    assert_eq!(
        storage.calls().len(),
        before + 1,
        "cached mapping skips probing"
    );
}

#[tokio::test]
async fn discovers_rev_prefix() {
    let storage = FakeStorage::default().bucket(
        "pubsite_prod_rev_123",
        vec![(object("202610"), "1", october())],
    );
    let report = probe(storage).await;
    assert_eq!(report["status"], "ok");
    assert_eq!(report["resolution"]["bucket"], "pubsite_prod_rev_123");
}

#[tokio::test]
async fn both_candidates_relevant_is_ambiguous() {
    let storage = play_bucket().bucket(
        "pubsite_prod_rev_123",
        vec![(object("202610"), "1", october())],
    );
    let report = probe(storage).await;
    assert_eq!(report["status"], "error");
    assert_eq!(report["error"]["code"], "ambiguous");
}

#[tokio::test]
async fn denied_candidate_stays_visible_when_other_succeeds() {
    let storage = FakeStorage::default()
        .failing(
            "pubsite_prod_123",
            StorageError::PermissionDenied("HTTP 403: no storage.objects.list".into()),
        )
        .bucket(
            "pubsite_prod_rev_123",
            vec![(object("202610"), "1", october())],
        );
    let report = probe(storage).await;
    assert_eq!(report["status"], "ok");
    assert_eq!(
        report["discovery"]["candidates"][0]["state"],
        "permission_denied"
    );
}

#[tokio::test]
async fn empty_prefix_is_access_without_relevance() {
    let storage = FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![(object("202610").replace(PKG, "other.app"), "1", vec![])],
    );
    let report = probe(storage).await;
    assert_eq!(report["status"], "no_data");
    assert_eq!(report["access"], "accessible_empty");
    assert_eq!(report["resolution"]["packageReportsSeen"], false);
}

#[tokio::test]
async fn discovery_failures_have_specific_codes() {
    let cases = [
        (FakeStorage::default(), "override_required"),
        (
            FakeStorage::default()
                .failing(
                    "pubsite_prod_123",
                    StorageError::PermissionDenied("HTTP 403".into()),
                )
                .failing(
                    "pubsite_prod_rev_123",
                    StorageError::BucketNotFound("HTTP 404".into()),
                ),
            "permission_denied",
        ),
        (
            FakeStorage::default()
                .failing("pubsite_prod_123", StorageError::Auth("HTTP 401".into())),
            "auth_failed",
        ),
    ];
    for (storage, code) in cases {
        let report = probe(storage).await;
        assert_eq!(report["error"]["code"], code);
        assert!(report["error"]["remedy"].is_string());
    }
}

#[tokio::test]
async fn missing_developer_id_and_explicit_override() {
    let storage = Arc::new(play_bucket());
    let ctx = context(ReportsConfig::default(), storage.clone());
    let report = run(&ctx, ACCESS, json!({"packageName": PKG})).await;
    assert_eq!(report["error"]["code"], "missing_developer_id");
    let report = run(
        &ctx,
        ACCESS,
        json!({"packageName": PKG, "bucket": "gs://pubsite_prod_123"}),
    )
    .await;
    assert_eq!(report["status"], "ok");
    assert_eq!(report["resolution"]["provenance"], "explicit_override");
    let report = run(
        &ctx,
        ACCESS,
        json!({"packageName": PKG, "bucket": "gs://pubsite_prod_123/stats"}),
    )
    .await;
    assert_eq!(report["error"]["code"], "invalid_bucket");
}

#[tokio::test]
async fn configured_bucket_skips_probing() {
    let storage = Arc::new(play_bucket());
    let ctx = context(
        ReportsConfig::from_vars(None, None, Some("gs://pubsite_prod_123")),
        storage.clone(),
    );
    let listed = run(&ctx, "reports_installs_list", json!({"packageName": PKG})).await;
    assert_eq!(listed["resolution"]["provenance"], "configured_bucket");
    assert_eq!(storage.calls().len(), 1);
}

#[tokio::test]
async fn transient_errors_retry_but_permission_denials_do_not() {
    let storage = Arc::new(play_bucket());
    *storage.transient_failures.lock().unwrap() = 2;
    let report = probe(storage).await;
    assert_eq!(report["status"], "ok");
    assert_eq!(report["sourceCalls"][0]["attempts"], 3);

    let storage = FakeStorage::default().failing(
        "pubsite_prod_123",
        StorageError::PermissionDenied("HTTP 403".into()),
    );
    let report = probe(storage).await;
    assert_eq!(report["sourceCalls"][0]["attempts"], 1);
}

#[tokio::test]
async fn unsupported_auth_fails_only_the_install_tools() {
    let ctx = Arc::new(InstallsContext::new(dev("123"), StorageSource::Unsupported));
    let report = run(&ctx, ACCESS, json!({"packageName": PKG})).await;
    assert_eq!(report["error"]["code"], "auth_unsupported");
    // Existing report tools are built independently and still register.
    assert_eq!(super::super::build_tools().len(), 5);
}

// ---------- query ----------

#[tokio::test]
async fn query_spans_months_and_separates_event_from_snapshot() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let report = run(
        &ctx,
        "reports_installs_query",
        json!({
            "packageName": PKG, "startDate": "2026-09-30", "endDate": "2026-10-02",
        }),
    )
    .await;
    assert_eq!(report["status"], "ok", "{report:#}");
    assert_eq!(report["sources"].as_array().unwrap().len(), 2);
    let v25 = &report["summaries"][0];
    assert_eq!(v25["dimensionValue"], "25");
    let metric = |summary: &Value, key: &str| {
        summary["metrics"]
            .as_array()
            .unwrap()
            .iter()
            .find(|m| m["metric"] == key)
            .unwrap()
            .clone()
    };
    // 2026-09-29 is outside the range: 8 + 7, not 10 + 8 + 7.
    assert_eq!(metric(v25, "dailyDeviceInstalls")["value"], 15);
    assert_eq!(
        metric(v25, "dailyDeviceInstalls")["aggregation"],
        "sum_over_observed_dates"
    );
    let current = metric(v25, "currentDeviceInstalls");
    assert_eq!(current["aggregation"], "latest_snapshot");
    assert_eq!(current["value"], 510);
    assert_eq!(current["asOfDate"], "2026-10-01");
    assert_eq!(report["coverage"]["observedFrom"], "2026-09-30");
    assert_eq!(report["timeZone"]["status"], "unknown");
    let row_id = report["rows"][0]["reportId"].as_str().unwrap();
    assert!(report["sources"]
        .as_array()
        .unwrap()
        .iter()
        .any(|s| s["reportId"] == row_id));
}

#[tokio::test]
async fn query_version_filter_observed_zero_blank_and_absent_version() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let report = run(&ctx, "reports_installs_query", json!({
        "packageName": PKG, "startDate": "2026-10-01", "endDate": "2026-10-04",
        "dimensionValues": ["26", 27], "metrics": ["dailyDeviceInstalls", "dailyDeviceUpgrades"],
    })).await;
    assert_eq!(report["summaries"].as_array().unwrap().len(), 1);
    let rows = report["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(
        rows[1]["values"]["dailyDeviceInstalls"], 0,
        "observed zero stays zero"
    );
    assert_eq!(
        rows[1]["values"]["dailyDeviceUpgrades"],
        Value::Null,
        "blank is not zero"
    );
    let upgrades = &report["summaries"][0]["metrics"][1];
    assert_eq!(upgrades["value"], 40);
    assert_eq!(upgrades["blankCells"], 1);
    assert_eq!(
        report["coverage"]["requestedVersionsNotObserved"],
        json!(["27"])
    );
    assert_eq!(
        report["coverage"]["datesWithoutObservations"],
        json!(["2026-10-03", "2026-10-04"])
    );
    assert_eq!(report["status"], "partial");
}

#[tokio::test]
async fn query_reports_missing_month_and_no_data() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let report = run(&ctx, "reports_installs_query", json!({
        "packageName": PKG, "startDate": "2026-10-30", "endDate": "2026-11-02", "dimensionValues": ["99"],
    })).await;
    assert_eq!(report["status"], "no_data");
    assert_eq!(report["coverage"]["missingMonths"], json!(["2026-11"]));
    assert!(report["warnings"]
        .to_string()
        .contains("does not mean zero installs"));
}

#[tokio::test]
async fn query_schema_failure_is_partial_and_raw_still_works() {
    let broken = utf16le(&format!("Date,Package Name,Version\n2026-10-01,{PKG},26\n"));
    let storage = play_bucket().bucket(
        "pubsite_prod_123",
        vec![
            (object("202609"), "11", september()),
            (object("202610"), "22", broken.clone()),
        ],
    );
    let ctx = context(dev("123"), Arc::new(storage));
    let report = run(
        &ctx,
        "reports_installs_query",
        json!({
            "packageName": PKG, "startDate": "2026-09-30", "endDate": "2026-10-01",
        }),
    )
    .await;
    assert_eq!(report["status"], "partial");
    let failed = &report["sources"][1];
    assert_eq!(failed["state"], "schema_error");
    assert!(failed["error"]["message"]
        .as_str()
        .unwrap()
        .contains("App Version Code"));

    let raw = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": failed["reportId"], "mode": "rows"}),
    )
    .await;
    assert_eq!(raw["headers"], json!(["Date", "Package Name", "Version"]));
    assert_eq!(raw["rows"][0]["cells"], json!(["2026-10-01", PKG, "26"]));
}

#[tokio::test]
async fn query_validates_requests() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    for arguments in [
        json!({"packageName": PKG, "startDate": "2026-10-05", "endDate": "2026-10-04"}),
        json!({"packageName": PKG, "startDate": "2026-01-01", "endDate": "2026-06-01"}),
        json!({"packageName": PKG, "startDate": "2026-10-01", "endDate": "2026-10-02", "dimension": "country"}),
        json!({"packageName": "x/../y", "startDate": "2026-10-01", "endDate": "2026-10-02"}),
    ] {
        assert_eq!(
            run(&ctx, "reports_installs_query", arguments).await["error"]["code"],
            "invalid_request"
        );
    }
    let date =
        |text: &str| mcp_factory_core::chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d").unwrap();
    assert_eq!(
        super::query::months(date("2025-12-31"), date("2026-01-01")),
        vec![(2025, 12), (2026, 1)]
    );
}

// ---------- listing and raw retrieval ----------

#[tokio::test]
async fn list_paginates_and_filters() {
    let storage = play_bucket().bucket(
        "pubsite_prod_123",
        vec![
            (object("202609"), "11", september()),
            (object("202610"), "21", october()),
            (
                object("202610").replace("app_version", "country"),
                "31",
                vec![1],
            ),
        ],
    );
    let ctx = context(
        ReportsConfig::from_vars(None, None, Some("pubsite_prod_123")),
        Arc::new(storage),
    );
    let page = run(
        &ctx,
        "reports_installs_list",
        json!({"packageName": PKG, "limit": 2}),
    )
    .await;
    assert_eq!(page["reports"].as_array().unwrap().len(), 2);
    assert_eq!(page["nextPageToken"], "2");
    let report = &page["reports"][0];
    assert_eq!(report["month"], "2026-09");
    assert_eq!(report["generation"], "11");
    assert_eq!(
        report["objectUri"],
        format!("gs://pubsite_prod_123/{}", object("202609"))
    );
    let filtered = run(
        &ctx,
        "reports_installs_list",
        json!({"packageName": PKG, "fromMonth": "2026-10", "dimension": "country"}),
    )
    .await;
    assert_eq!(filtered["reports"].as_array().unwrap().len(), 1);
    // The month range is applied by Storage, so later months are not hidden
    // behind a first page full of earlier ones.
    let bounded = run(
        &ctx,
        "reports_installs_list",
        json!({"packageName": PKG, "fromMonth": "2026-10", "toMonth": "2026-10", "limit": 1}),
    )
    .await;
    assert_eq!(bounded["reports"][0]["month"], "2026-10");
    assert_eq!(super::next_month("2026-12").as_deref(), Some("202701"));
}

#[tokio::test]
async fn original_csv_is_byte_exact_with_sha256() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let listed = run(&ctx, "reports_installs_list", json!({"packageName": PKG})).await;
    let id = listed["reports"][1]["reportId"].clone();
    let result = call(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "mode": "original_csv"}),
    )
    .await;
    let ToolBody::Binary { data, mime } = &result.body else {
        panic!("expected binary body")
    };
    assert_eq!(*data, october());
    assert_eq!(mime, "text/csv");
    let structured = result.structured.unwrap();
    assert_eq!(
        structured["sha256"],
        format!("{:x}", Sha256::digest(october()))
    );
    assert_eq!(structured["encoding"], "UTF-16LE (BOM)");
    assert_eq!(structured["byteSize"], october().len());
}

#[tokio::test]
async fn raw_rows_paginate_as_source_strings() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let listed = run(&ctx, "reports_installs_list", json!({"packageName": PKG})).await;
    let id = listed["reports"][1]["reportId"].clone();
    let first = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "limit": 2}),
    )
    .await;
    assert_eq!(first["label"], "parsed_source_rows");
    assert_eq!(first["totalRows"], 3);
    assert_eq!(first["nextOffset"], 2);
    let second = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "offset": 2, "limit": 2}),
    )
    .await;
    assert_eq!(second["rows"][0]["rowNumber"], 3);
    assert_eq!(
        second["rows"][0]["cells"][4], "",
        "blank cell kept as empty string"
    );
    assert_eq!(second["nextOffset"], Value::Null);
}

#[tokio::test]
async fn raw_retrieval_pins_generation() {
    let storage = Arc::new(play_bucket());
    let ctx = context(dev("123"), storage.clone());
    let listed = run(&ctx, "reports_installs_list", json!({"packageName": PKG})).await;
    let id = listed["reports"][1]["reportId"].clone();
    storage.buckets.lock().unwrap().insert(
        "pubsite_prod_123".to_string(),
        Ok(vec![
            (object("202609"), "11".into(), september()),
            (object("202610"), "22".into(), october()),
        ]),
    );
    let raw = run(&ctx, "reports_installs_get_raw", json!({"reportId": id})).await;
    assert_eq!(raw["error"]["code"], "source_changed");
    assert_eq!(raw["currentGeneration"], "22");
}

#[tokio::test]
async fn raw_rejects_foreign_and_out_of_scope_ids() {
    let ctx = context(dev("123"), Arc::new(play_bucket()));
    let id = |bucket: &str, object: &str| super::report_id(bucket, &meta(object, "1", &[]));
    let foreign = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id("pubsite_prod_999", &object("202610"))}),
    )
    .await;
    assert_eq!(foreign["error"]["code"], "account_mismatch");
    let financial = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id("pubsite_prod_123", "earnings/earnings_202609.zip")}),
    )
    .await;
    assert_eq!(financial["error"]["code"], "out_of_scope");
    let garbage = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": "gs://pubsite_prod_123/x"}),
    )
    .await;
    assert_eq!(garbage["error"]["code"], "invalid_report_id");
    assert!(parse_report_id(&id("b", "o")).is_some());
}

// ---------- HTTP client ----------

#[tokio::test]
async fn gcs_client_maps_statuses_and_pins_generation() {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;
    let encoded = "stats%2Finstalls%2Finstalls_a_202610_app_version.csv";
    Mock::given(method("GET"))
        .and(path("/b/pubsite_prod_1/o"))
        .and(query_param("prefix", "stats/installs/installs_a_"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "items": [{"name": "stats/installs/installs_a_202610_app_version.csv", "generation": "7", "size": "4"}],
            "nextPageToken": "n"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/b/pubsite_prod_2/o"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(
                json!({"error": {"message": "The specified bucket does not exist."}}),
            ),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/b/pubsite_prod_3/o"))
        .respond_with(
            ResponseTemplate::new(403).set_body_json(json!({"error": {"message": "denied"}})),
        )
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path(format!("/b/pubsite_prod_1/o/{encoded}")))
        .and(query_param("alt", "media"))
        .and(query_param("generation", "7"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(vec![0xFF, 0xFE, 0x41, 0x00]))
        .mount(&server)
        .await;

    let client = GcsStorage::unauthenticated(&server.uri());
    let page = client
        .list(
            "pubsite_prod_1",
            "stats/installs/installs_a_",
            (None, None),
            None,
            10,
        )
        .await
        .unwrap();
    assert_eq!(page.items[0].generation, "7");
    assert_eq!(page.next_page_token.as_deref(), Some("n"));
    assert!(matches!(
        client
            .list("pubsite_prod_2", "p", (None, None), None, 1)
            .await,
        Err(StorageError::BucketNotFound(_))
    ));
    assert!(matches!(
        client
            .list("pubsite_prod_3", "p", (None, None), None, 1)
            .await,
        Err(StorageError::PermissionDenied(_))
    ));
    let object = "stats/installs/installs_a_202610_app_version.csv";
    assert_eq!(
        client
            .download("pubsite_prod_1", object, "7", 10)
            .await
            .unwrap(),
        vec![0xFF, 0xFE, 0x41, 0x00]
    );
    assert!(matches!(
        client.download("pubsite_prod_1", object, "7", 2).await,
        Err(StorageError::TooLarge { .. })
    ));
    assert!(matches!(
        client.download("pubsite_prod_1", object, "8", 10).await,
        Err(StorageError::ObjectNotFound(_))
    ));
}

#[test]
fn live_header_layout_maps_every_column() {
    // Header row of a real app_version export observed on 2026-10-05.
    let header = "Date,Package name,App Version Code,Daily Device Installs,Daily Device Uninstalls,Daily Device Upgrades,Total User Installs,Daily User Installs,Daily User Uninstalls,Active Device Installs,Install events,Update events,Uninstall events";
    let text = format!("{header}\n2026-09-25,{PKG},25,11,0,0,0,11,10,87,11,0,9\n");
    let parsed = parse_installs(&text, PKG, None).unwrap();
    assert_eq!(parsed.metrics.len(), 10);
    assert_eq!(parsed.rows[0].values["activeDeviceInstalls"], Some(87));
    assert_eq!(parsed.rows[0].values["uninstallEvents"], Some(9));
}

// ---------- failure paths ----------

#[tokio::test]
async fn access_failure_evicts_cached_mapping() {
    let storage = Arc::new(play_bucket());
    let ctx = context(dev("123"), storage.clone());
    let list = || run(&ctx, "reports_installs_list", json!({"packageName": PKG}));
    assert_eq!(
        list().await["resolution"]["provenance"],
        "probe:pubsite_prod"
    );
    assert_eq!(
        list().await["resolution"]["provenance"],
        "cache(probe:pubsite_prod)"
    );

    let healthy = storage
        .buckets
        .lock()
        .unwrap()
        .remove("pubsite_prod_123")
        .unwrap();
    storage.buckets.lock().unwrap().insert(
        "pubsite_prod_123".into(),
        Err(StorageError::PermissionDenied("HTTP 403".into())),
    );
    assert_eq!(list().await["error"]["code"], "permission_denied");

    storage
        .buckets
        .lock()
        .unwrap()
        .insert("pubsite_prod_123".into(), healthy);
    assert_eq!(
        list().await["resolution"]["provenance"],
        "probe:pubsite_prod",
        "the mapping was dropped, so the bucket is probed again"
    );
}

async fn query(storage: FakeStorage) -> Value {
    let ctx = context(dev("123"), Arc::new(storage));
    run(
        &ctx,
        "reports_installs_query",
        json!({"packageName": PKG, "startDate": "2026-09-30", "endDate": "2026-10-01"}),
    )
    .await
}

#[tokio::test]
async fn query_keeps_evidence_when_one_month_fails() {
    let report = query(play_bucket().fail_call(
        &format!("metadata {}", object("202609")),
        StorageError::Other("HTTP 400".into()),
    ))
    .await;
    assert_eq!(report["status"], "partial");
    assert_eq!(report["sources"][0]["state"], "metadata_failed");
    assert_eq!(report["coverage"]["failedMonths"][0]["month"], "2026-09");
    assert_eq!(
        report["rows"].as_array().unwrap().len(),
        2,
        "October rows survive"
    );

    let report = query(play_bucket().fail_call(
        &format!("download {}", object("202610")),
        StorageError::Other("HTTP 400".into()),
    ))
    .await;
    assert_eq!(report["status"], "partial");
    assert_eq!(report["sources"][1]["state"], "download_failed");
    assert_eq!(report["coverage"]["failedMonths"][0]["month"], "2026-10");
}

#[tokio::test]
async fn query_is_error_when_every_month_fails() {
    let report =
        query(play_bucket().fail_call("download ", StorageError::Other("HTTP 400".into()))).await;
    assert_eq!(report["status"], "error");
    assert_eq!(report["error"]["code"], "sources_unavailable");
    assert_eq!(report["error"]["failedMonths"].as_array().unwrap().len(), 2);
    assert!(
        !report["warnings"]
            .to_string()
            .contains("does not mean zero"),
        "no empty-result reassurance on a hard error"
    );
}

#[tokio::test]
async fn query_reports_rejected_rows_without_counting_them() {
    let october = utf16le(&csv(&[
        &row("2026-10-01", "26", "3,40,43,45"),
        &row("2026-10-01", "27", "x,1,1,1"),
    ]));
    let storage = FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![
            (object("202609"), "11", september()),
            (object("202610"), "21", october),
        ],
    );
    let report = query(storage).await;
    let source = &report["sources"][1];
    assert_eq!(source["rejectedRows"], 1);
    assert!(source["issues"][0]["reason"]
        .as_str()
        .unwrap()
        .contains("invalid integer"));
    assert_eq!(report["status"], "partial");
    assert!(report["warnings"]
        .to_string()
        .contains("rejected, not counted"));
    assert!(report["rows"]
        .as_array()
        .unwrap()
        .iter()
        .all(|row| row["dimensionValue"] != "27"));
}

fn raw_context(storage: FakeStorage) -> Arc<InstallsContext> {
    context(
        ReportsConfig::from_vars(None, None, Some("pubsite_prod_123")),
        Arc::new(storage),
    )
}

fn raw_id(month: &str, generation: &str, bytes: &[u8]) -> Value {
    json!(super::report_id(
        "pubsite_prod_123",
        &meta(&object(month), generation, bytes)
    ))
}

#[tokio::test]
async fn raw_original_csv_over_host_limit_is_refused() {
    let big = vec![b'a'; super::MAX_ORIGINAL_BYTES as usize + 1];
    let ctx = raw_context(FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![(object("202610"), "1", big.clone())],
    ));
    let id = raw_id("202610", "1", &big);
    let raw = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "mode": "original_csv"}),
    )
    .await;
    assert_eq!(raw["error"]["code"], "too_large_for_host");
    assert!(raw["error"]["remedy"]
        .as_str()
        .unwrap()
        .contains("no public objects or signed URLs"));
}

#[tokio::test]
async fn raw_download_vanishing_mid_read_is_source_changed() {
    let ctx = raw_context(play_bucket().fail_call(
        "download ",
        StorageError::ObjectNotFound("HTTP 404: No such object".into()),
    ));
    let raw = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": raw_id("202610", "21", &october())}),
    )
    .await;
    assert_eq!(raw["error"]["code"], "source_changed");
}

#[tokio::test]
async fn undecodable_export_fails_rows_but_original_bytes_remain() {
    let garbage = vec![0xC3, 0x28, 0x41];
    let ctx = raw_context(FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![(object("202610"), "1", garbage.clone())],
    ));
    let id = raw_id("202610", "1", &garbage);
    let rows = run(&ctx, "reports_installs_get_raw", json!({"reportId": id})).await;
    assert_eq!(rows["error"]["code"], "parse_failed");
    assert!(rows["error"]["remedy"]
        .as_str()
        .unwrap()
        .contains("original_csv"));
    assert_eq!(rows["source"]["generation"], "1");

    let result = call(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "mode": "original_csv"}),
    )
    .await;
    let ToolBody::Binary { data, .. } = &result.body else {
        panic!("expected binary body")
    };
    assert_eq!(*data, garbage);
    assert_eq!(result.structured.unwrap()["encoding"], "undetected");
}

#[tokio::test]
async fn gcs_client_metadata_and_error_classes() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    let server = MockServer::start().await;
    let mount = |bucket: &'static str, response: ResponseTemplate| {
        Mock::given(method("GET"))
            .and(path(format!("/b/{bucket}/o")))
            .respond_with(response)
    };
    mount("unauthorized", ResponseTemplate::new(401))
        .mount(&server)
        .await;
    mount("throttled", ResponseTemplate::new(429))
        .mount(&server)
        .await;
    mount(
        "broken",
        ResponseTemplate::new(503).set_body_string("upstream down"),
    )
    .mount(&server)
    .await;
    mount("teapot", ResponseTemplate::new(418))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/b/pubsite_prod_1/o/stats%2Fa.csv"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "name": "stats/a.csv", "generation": "9", "size": "12",
            "updated": "2026-10-03T21:01:27Z", "md5Hash": "m", "crc32c": "c", "contentEncoding": "gzip"
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/b/pubsite_prod_1/o/stats%2Fpartial.csv"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"name": "stats/partial.csv"})),
        )
        .mount(&server)
        .await;

    let client = GcsStorage::unauthenticated(&server.uri());
    assert_eq!(client.principal(), "test-principal");
    let list = |bucket: &'static str| client.list(bucket, "p", (None, None), None, 1);
    assert!(matches!(
        list("unauthorized").await,
        Err(StorageError::Auth(_))
    ));
    assert!(matches!(
        list("throttled").await,
        Err(StorageError::Transient(_))
    ));
    let Err(StorageError::Transient(message)) = list("broken").await else {
        panic!("503 is transient")
    };
    assert_eq!(message, "HTTP 503: upstream down");
    assert!(matches!(list("teapot").await, Err(StorageError::Other(_))));

    let meta = client
        .metadata("pubsite_prod_1", "stats/a.csv")
        .await
        .unwrap();
    assert_eq!((meta.generation.as_str(), meta.size), ("9", 12));
    assert_eq!(meta.content_encoding.as_deref(), Some("gzip"));
    assert!(matches!(
        client.metadata("pubsite_prod_1", "stats/partial.csv").await,
        Err(StorageError::Other(_))
    ));
}

#[test]
fn storage_errors_have_stable_codes_and_messages() {
    let too_large = StorageError::TooLarge { size: 10, limit: 5 };
    assert_eq!(too_large.code(), "too_large");
    assert_eq!(
        too_large.message(),
        "object is 10 bytes; the limit is 5 bytes"
    );
    assert_eq!(
        too_large.to_json(),
        json!({"code": "too_large", "message": "object is 10 bytes; the limit is 5 bytes"})
    );
    assert_eq!(
        StorageError::Transient("t".into()).code(),
        "storage_unavailable"
    );
    assert_eq!(StorageError::Other("o".into()).code(), "storage_error");
}

// ---------- review regressions ----------

#[tokio::test]
async fn probe_ignores_packages_that_extend_the_name() {
    // `org.example.app_beta` shares the listing prefix of `org.example.app`.
    let sibling = format!("stats/installs/installs_{PKG}_beta_202610_app_version.csv");
    let storage = FakeStorage::default()
        .bucket("pubsite_prod_123", vec![(sibling, "1", vec![])])
        .bucket(
            "pubsite_prod_rev_123",
            vec![(object("202610"), "1", october())],
        );
    let report = probe(storage).await;
    assert_eq!(report["status"], "ok", "{report:#}");
    assert_eq!(report["resolution"]["bucket"], "pubsite_prod_rev_123");
    assert_eq!(
        report["discovery"]["candidates"][0]["state"],
        "accessible_empty"
    );
}

#[tokio::test]
async fn cached_mapping_is_per_package() {
    let other = "org.example.other";
    let storage = FakeStorage::default()
        .bucket("pubsite_prod_123", vec![(object("202610"), "1", october())])
        .bucket(
            "pubsite_prod_rev_123",
            vec![(object("202610").replace(PKG, other), "1", vec![])],
        );
    let ctx = context(dev("123"), Arc::new(storage));
    let list = |package: &str| {
        run(
            &ctx,
            "reports_installs_list",
            json!({"packageName": package}),
        )
    };
    assert_eq!(list(PKG).await["resolution"]["bucket"], "pubsite_prod_123");
    let second = list(other).await;
    assert_eq!(
        second["resolution"]["provenance"], "probe:pubsite_prod_rev",
        "{other} is probed on its own, not served from {PKG}'s mapping"
    );
    assert_eq!(second["resolution"]["bucket"], "pubsite_prod_rev_123");
}

#[tokio::test]
async fn cached_object_still_respects_the_inline_limit() {
    // Stored (gzip) size is tiny, delivered CSV exceeds the original_csv limit.
    let big = vec![b'a'; super::MAX_ORIGINAL_BYTES as usize + 1];
    let storage = FakeStorage::default().bucket(
        "pubsite_prod_123",
        vec![(object("202610"), "1", big.clone())],
    );
    *storage.stored_size.lock().unwrap() = Some(1024);
    let ctx = raw_context(storage);
    let id = raw_id("202610", "1", &big);
    // rows mode allows up to MAX_PARSE_BYTES and caches the object...
    let rows = run(&ctx, "reports_installs_get_raw", json!({"reportId": id})).await;
    assert_eq!(rows["status"], "ok");
    // ...which must not let original_csv skip its smaller limit.
    let original = run(
        &ctx,
        "reports_installs_get_raw",
        json!({"reportId": id, "mode": "original_csv"}),
    )
    .await;
    assert_eq!(original["error"]["code"], "too_large_for_host");
}

#[test]
fn duplicate_used_header_is_a_schema_error() {
    let text = format!(
        "Date,Package Name,App Version Code,Daily Device Installs,Daily Device Installs\n2026-10-01,{PKG},26,1,99\n"
    );
    let error = parse_installs(&text, PKG, None).unwrap_err();
    assert!(
        error.message.contains("Daily Device Installs"),
        "{}",
        error.message
    );

    let tolerated = format!(
        "Date,Package Name,App Version Code,Daily Device Installs,Note,note\n2026-10-01,{PKG},26,1,a,b\n"
    );
    assert_eq!(parse_installs(&tolerated, PKG, None).unwrap().rows.len(), 1);
}

#[tokio::test]
async fn storage_initialization_failure_is_retried() {
    let variable = "GOOGLE_PLAY_MCP_TEST_RETRY_KEY";
    let ctx = Arc::new(InstallsContext::new(
        dev("123"),
        StorageSource::KeyEnv(variable.to_string()),
    ));
    let first = probe_ctx(&ctx).await;
    assert!(first["error"]["message"]
        .as_str()
        .unwrap()
        .contains("is not set"));

    // Fixing the configuration takes effect without a restart. Only this test
    // reads this variable, so setting it cannot race other tests.
    let missing = std::env::temp_dir().join("google-play-mcp-test-missing-key.json");
    std::env::set_var(variable, &missing);
    let second = probe_ctx(&ctx).await;
    std::env::remove_var(variable);
    assert_eq!(second["error"]["code"], "auth_failed");
    assert!(
        second["error"]["message"]
            .as_str()
            .unwrap()
            .contains("cannot read credential file"),
        "the second call re-read the configuration: {second:#}"
    );
}

async fn probe_ctx(ctx: &Arc<InstallsContext>) -> Value {
    run(ctx, ACCESS, json!({"packageName": PKG})).await
}
