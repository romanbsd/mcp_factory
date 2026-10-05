//! Optional install-report tools backed by Play's monthly Cloud Storage CSV
//! exports. Storage access is created lazily on first use and every failure is
//! returned as a structured envelope, so a missing configuration, denied
//! permission, or absent export never affects startup or the other tools.

mod parse;
mod query;
mod resolve;
mod storage;

use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use base64::Engine;
use mcp_factory_core::chrono::Utc;
use mcp_factory_core::{
    async_trait, AuthConfig, CustomToolHandler, CustomToolSpec, ProxyConfig, ProxyError,
    ReadOnlyToolInvoker, ToolBody, ToolResult,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::sync::OnceCell;

use super::client::{call_record, retry_delay, CallOutcome};
use super::{read_only_spec, REPORT_DEADLINE};
use resolve::{AccountConfig, Failure, ReportsConfig, Resolution, INSTALLS_PREFIX};
use storage::{GcsStorage, ListPage, ObjectMeta, Offsets, Storage, StorageError};

/// Largest export downloaded for interpretation.
const MAX_PARSE_BYTES: u64 = 20 * 1024 * 1024;
/// Largest export returned byte-for-byte in one tool result (base64 inflates it
/// by a third inside the MCP message).
const MAX_ORIGINAL_BYTES: u64 = 8 * 1024 * 1024;
const MAX_RAW_ROWS: usize = 500;
const OBJECT_CACHE_ENTRIES: usize = 8;
const REPORT_ID_PREFIX: &str = "gpir1_";

pub fn build_tools(config: &ProxyConfig) -> Vec<CustomToolSpec> {
    if !enabled(std::env::var("GOOGLE_PLAY_INSTALL_REPORTS").ok().as_deref()) {
        return Vec::new();
    }
    let source = match &config.auth {
        AuthConfig::GoogleServiceAccount {
            credentials_path_env,
            ..
        } => StorageSource::KeyEnv(credentials_path_env.clone()),
        _ => StorageSource::Unsupported,
    };
    tools(Arc::new(InstallsContext::new(
        ReportsConfig::from_env(),
        source,
    )))
}

fn enabled(value: Option<&str>) -> bool {
    !matches!(
        value
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref(),
        Some("off" | "0" | "false" | "disabled")
    )
}

fn tools(ctx: Arc<InstallsContext>) -> Vec<CustomToolSpec> {
    let account = json!({"type": "string", "description": "Developer id (or bucket) of a configured account; needed only when several are configured."});
    let package = json!({"type": "string", "minLength": 1});
    vec![
        spec(
            "reports_install_access_check",
            "Resolve and validate the Play install-report Cloud Storage bucket for a package, with per-candidate diagnostics (auth, permission, missing bucket, ambiguity, empty prefix).",
            json!({
                "type": "object",
                "properties": {
                    "packageName": package,
                    "account": account,
                    "bucket": {"type": "string", "description": "Explicit bucket override: bare name or gs:// URI from Play Console > Download reports > Statistics."}
                },
                "required": ["packageName"],
                "additionalProperties": false
            }),
            Kind::AccessCheck,
            &ctx,
        ),
        spec(
            "reports_installs_query",
            "Install statistics by version code from Play's monthly install exports (delayed 3-7 days; not real time). Daily event metrics are summed over observed dates; current/total metrics are latest-date snapshots. Missing dates are unobserved, never zero.",
            json!({
                "type": "object",
                "properties": {
                    "packageName": package,
                    "startDate": {"type": "string", "format": "date", "description": "Inclusive ISO date."},
                    "endDate": {"type": "string", "format": "date", "description": "Inclusive ISO date; at most 92 days after startDate."},
                    "dimension": {"enum": ["app_version"], "default": "app_version"},
                    "dimensionValues": {"type": "array", "maxItems": 50, "items": {"type": ["string", "integer"]}, "description": "Version codes to keep; omit for all."},
                    "metrics": {"type": "array", "items": {"enum": parse::METRICS.iter().map(|metric| metric.key).collect::<Vec<_>>()}, "description": "Omit for every metric present in the export."},
                    "includeRows": {"type": "boolean", "default": true},
                    "account": account
                },
                "required": ["packageName", "startDate", "endDate"],
                "additionalProperties": false
            }),
            Kind::Query,
            &ctx,
        ),
        spec(
            "reports_installs_list",
            "List a package's install-report exports with opaque report ids, generation, size, timestamps, and checksums.",
            json!({
                "type": "object",
                "properties": {
                    "packageName": package,
                    "fromMonth": {"type": "string", "pattern": "^\\d{4}-\\d{2}$"},
                    "toMonth": {"type": "string", "pattern": "^\\d{4}-\\d{2}$"},
                    "dimension": {"type": "string", "pattern": "^[a-z_]+$"},
                    "pageToken": {"type": "string"},
                    "limit": {"type": "integer", "minimum": 1, "maximum": 100, "default": 50},
                    "account": account
                },
                "required": ["packageName"],
                "additionalProperties": false
            }),
            Kind::List,
            &ctx,
        ),
        spec(
            "reports_installs_get_raw",
            "Retrieve one install export by report id, pinned to its generation: original_csv returns the exact bytes (with SHA-256) as an embedded resource; rows returns paginated, unnormalized string cells.",
            json!({
                "type": "object",
                "properties": {
                    "reportId": {"type": "string", "minLength": 1},
                    "mode": {"enum": ["original_csv", "rows"], "default": "rows"},
                    "offset": {"type": "integer", "minimum": 0, "default": 0, "description": "rows mode: zero-based data-row offset."},
                    "limit": {"type": "integer", "minimum": 1, "maximum": MAX_RAW_ROWS, "default": 200},
                    "account": account
                },
                "required": ["reportId"],
                "additionalProperties": false
            }),
            Kind::GetRaw,
            &ctx,
        ),
    ]
}

fn spec(
    name: &str,
    description: &str,
    input_schema: Value,
    kind: Kind,
    ctx: &Arc<InstallsContext>,
) -> CustomToolSpec {
    let handler = Arc::new(InstallsHandler {
        kind,
        ctx: ctx.clone(),
    });
    read_only_spec(name, description, input_schema, handler)
}

#[derive(Clone, Copy)]
enum Kind {
    AccessCheck,
    Query,
    List,
    GetRaw,
}

enum StorageSource {
    /// Service-account key path in this environment variable (shared with the
    /// Publisher tools).
    KeyEnv(String),
    Unsupported,
    #[cfg(test)]
    Ready(Arc<dyn Storage>),
}

pub struct InstallsContext {
    reports: ReportsConfig,
    source: StorageSource,
    /// Only a successfully built client is kept: a missing or bad key is
    /// retried on the next call, so fixing it needs no restart.
    storage: OnceCell<Arc<dyn Storage>>,
    /// Verified bucket per (account, principal, package). Relevance is proven
    /// per package, so one package's probe never vouches for another's.
    mappings: Mutex<HashMap<(String, String, String), Resolution>>,
    /// Downloaded exports keyed by bucket/object#generation, so a changed
    /// monthly file is a cache miss rather than stale data.
    // ponytail: FIFO of 8 objects; switch to LRU with a byte budget if raw paging gets heavy.
    objects: Mutex<VecDeque<(String, Arc<Vec<u8>>)>>,
}

impl InstallsContext {
    fn new(reports: ReportsConfig, source: StorageSource) -> Self {
        Self {
            reports,
            source,
            storage: OnceCell::new(),
            mappings: Mutex::new(HashMap::new()),
            objects: Mutex::new(VecDeque::new()),
        }
    }

    async fn storage(&self) -> Result<Arc<dyn Storage>, Failure> {
        self.storage
            .get_or_try_init(|| async {
                match &self.source {
                    StorageSource::KeyEnv(variable) => {
                        let path = std::env::var(variable)
                            .ok()
                            .filter(|path| !path.trim().is_empty())
                            .ok_or_else(|| {
                                Failure::new("auth_failed", format!("{variable} is not set"))
                            })?;
                        GcsStorage::from_key_file(&path)
                            .map(|storage| Arc::new(storage) as Arc<dyn Storage>)
                            .map_err(|error| Failure::from(&error))
                    }
                    StorageSource::Unsupported => Err(Failure::new(
                        "auth_unsupported",
                        "Install reports need google_service_account auth; the configured auth cannot be assumed to grant Cloud Storage access.",
                    )
                    .remedy("Configure [auth] type = \"google_service_account\" (see README, Install reports).")),
                    #[cfg(test)]
                    StorageSource::Ready(storage) => Ok(storage.clone()),
                }
            })
            .await
            .cloned()
    }

    /// Bucket for an account: explicit configuration wins, then a cached
    /// verified mapping, then probing (cached only when package reports were seen).
    async fn resolve(
        &self,
        storage: &dyn Storage,
        recorder: &Recorder,
        account: &AccountConfig,
        package: &str,
    ) -> (Result<Resolution, Failure>, Vec<Value>) {
        if let Some(bucket) = &account.bucket {
            let resolution = Resolution {
                bucket: bucket.clone(),
                provenance: "configured_bucket".to_string(),
                verified_at: None,
                relevant: None,
            };
            return (Ok(resolution), Vec::new());
        }
        let key = mapping_key(account, storage, package);
        if let Some(cached) = self.mappings.lock().unwrap().get(&key).cloned() {
            let provenance = format!("cache({})", cached.provenance);
            return (
                Ok(Resolution {
                    provenance,
                    ..cached
                }),
                Vec::new(),
            );
        }
        let outcome = resolve::probe(
            storage,
            recorder,
            resolve::candidates(account, None),
            package,
        )
        .await;
        if let Ok(resolution) = &outcome.result {
            self.remember(account, storage, package, resolution);
        }
        (outcome.result, outcome.candidates)
    }

    /// Caches a mapping only when probing (not an explicit bucket) found this
    /// package's reports; an empty-but-accessible bucket proves too little.
    fn remember(
        &self,
        account: &AccountConfig,
        storage: &dyn Storage,
        package: &str,
        resolution: &Resolution,
    ) {
        if resolution.relevant == Some(true) && resolution.provenance.starts_with("probe:") {
            self.mappings
                .lock()
                .unwrap()
                .insert(mapping_key(account, storage, package), resolution.clone());
        }
    }

    /// Drops a cached mapping after an access failure so the next call probes
    /// again instead of trusting a mapping that stopped working.
    fn evict(
        &self,
        account: &AccountConfig,
        storage: &dyn Storage,
        package: &str,
        failed: &StorageError,
    ) {
        if matches!(
            failed,
            StorageError::PermissionDenied(_)
                | StorageError::BucketNotFound(_)
                | StorageError::Auth(_)
        ) {
            self.mappings
                .lock()
                .unwrap()
                .remove(&mapping_key(account, storage, package));
        }
    }

    async fn download(
        &self,
        storage: &dyn Storage,
        recorder: &Recorder,
        bucket: &str,
        meta: &ObjectMeta,
        max_bytes: u64,
    ) -> Result<Arc<Vec<u8>>, StorageError> {
        if meta.size > max_bytes {
            return Err(StorageError::TooLarge {
                size: meta.size,
                limit: max_bytes,
            });
        }
        let key = format!("{bucket}/{}#{}", meta.name, meta.generation);
        if let Some((_, bytes)) = self.objects.lock().unwrap().iter().find(|(k, _)| *k == key) {
            // `meta.size` is the stored size, which for a gzip-encoded export is
            // the compressed size; the limit applies to the delivered bytes.
            if bytes.len() as u64 > max_bytes {
                return Err(StorageError::TooLarge {
                    size: bytes.len() as u64,
                    limit: max_bytes,
                });
            }
            return Ok(bytes.clone());
        }
        let bytes = Arc::new(recorder.download(storage, bucket, meta, max_bytes).await?);
        let mut objects = self.objects.lock().unwrap();
        objects.push_back((key, bytes.clone()));
        if objects.len() > OBJECT_CACHE_ENTRIES {
            objects.pop_front();
        }
        Ok(bytes)
    }
}

fn mapping_key(
    account: &AccountConfig,
    storage: &dyn Storage,
    package: &str,
) -> (String, String, String) {
    (
        account.key(),
        storage.principal().to_string(),
        package.to_string(),
    )
}

/// Records every Storage call in the same `sourceCalls` shape as the other
/// report tools, retrying only transient failures (never auth or permission).
#[derive(Default)]
pub struct Recorder {
    calls: Mutex<Vec<Value>>,
}

impl Recorder {
    pub async fn list(
        &self,
        storage: &dyn Storage,
        bucket: &str,
        prefix: &str,
        offsets: Offsets<'_>,
        page_token: Option<&str>,
        max_results: u32,
    ) -> Result<ListPage, StorageError> {
        let arguments = json!({"bucket": bucket, "prefix": prefix, "startOffset": offsets.0,
            "endOffset": offsets.1, "pageToken": page_token, "maxResults": max_results});
        self.call(
            &format!("list-{bucket}"),
            "storage.objects.list",
            arguments,
            || storage.list(bucket, prefix, offsets, page_token, max_results),
        )
        .await
    }

    pub async fn metadata(
        &self,
        storage: &dyn Storage,
        bucket: &str,
        object: &str,
    ) -> Result<ObjectMeta, StorageError> {
        let arguments = json!({"bucket": bucket, "object": object});
        self.call(
            &format!("metadata-{object}"),
            "storage.objects.get",
            arguments,
            || storage.metadata(bucket, object),
        )
        .await
    }

    async fn download(
        &self,
        storage: &dyn Storage,
        bucket: &str,
        meta: &ObjectMeta,
        max_bytes: u64,
    ) -> Result<Vec<u8>, StorageError> {
        let arguments =
            json!({"bucket": bucket, "object": meta.name, "generation": meta.generation});
        self.call(
            &format!("download-{}", meta.name),
            "storage.objects.get(alt=media)",
            arguments,
            || storage.download(bucket, &meta.name, &meta.generation, max_bytes),
        )
        .await
    }

    async fn call<T, F, Fut>(
        &self,
        id: &str,
        method: &str,
        arguments: Value,
        operation: F,
    ) -> Result<T, StorageError>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<T, StorageError>>,
    {
        let started = Instant::now();
        let started_at = Utc::now().to_rfc3339();
        let mut attempts = 0;
        loop {
            attempts += 1;
            let result = operation().await;
            if matches!(result, Err(StorageError::Transient(_))) && attempts < 3 {
                retry_delay(attempts, &Value::Null).await;
                continue;
            }
            let (result_state, error) = match &result {
                Ok(_) => ("success", None),
                Err(error) => ("error", Some(error.to_json())),
            };
            self.calls.lock().unwrap().push(call_record(
                id,
                method,
                &arguments,
                &started_at,
                started.elapsed(),
                CallOutcome {
                    attempts,
                    result_state,
                    pagination_complete: true,
                    error,
                },
            ));
            return result;
        }
    }

    fn calls(&self) -> Vec<Value> {
        self.calls.lock().unwrap().clone()
    }
}

struct Reply {
    value: Value,
    original: Option<Vec<u8>>,
}

impl From<Value> for Reply {
    fn from(value: Value) -> Self {
        Self {
            value,
            original: None,
        }
    }
}

struct InstallsHandler {
    kind: Kind,
    ctx: Arc<InstallsContext>,
}

#[async_trait]
impl CustomToolHandler for InstallsHandler {
    async fn call(
        &self,
        _invoker: &dyn ReadOnlyToolInvoker,
        arguments: Value,
    ) -> Result<ToolResult, ProxyError> {
        let recorder = Recorder::default();
        let work = async {
            match self.kind {
                Kind::AccessCheck => access_check(&self.ctx, &recorder, &arguments).await.into(),
                Kind::Query => query::run(&self.ctx, &recorder, &arguments).await.into(),
                Kind::List => list(&self.ctx, &recorder, &arguments).await.into(),
                Kind::GetRaw => get_raw(&self.ctx, &recorder, &arguments).await,
            }
        };
        let mut reply = match tokio::time::timeout(REPORT_DEADLINE, work).await {
            Ok(reply) => reply,
            Err(_) => failure(Failure::new(
                "deadline_exceeded",
                "The reporting deadline was exhausted; completed Storage calls are listed in sourceCalls.",
            ))
            .into(),
        };
        // Envelope fields every response carries, filled once here.
        if let Some(envelope) = reply.value.as_object_mut() {
            envelope
                .entry("retrievedAt")
                .or_insert_with(|| json!(Utc::now().to_rfc3339()));
            envelope.entry("warnings").or_insert_with(|| json!([]));
            envelope.insert("sourceCalls".to_string(), Value::Array(recorder.calls()));
        }
        let structured = reply.value;
        let body = match reply.original {
            Some(data) => ToolBody::Binary {
                data,
                mime: "text/csv".to_string(),
            },
            None => {
                ToolBody::Text(serde_json::to_string_pretty(&structured).map_err(|error| {
                    ProxyError::Other(format!("failed to encode report: {error}"))
                })?)
            }
        };
        Ok(ToolResult {
            body,
            structured: Some(structured),
            meta: Default::default(),
            is_error: false,
        })
    }
}

/// Error envelope; callers add context fields by index (`value["source"] = ...`).
fn failure(failure: impl Into<Failure>) -> Value {
    json!({"status": "error", "error": failure.into().to_json()})
}

fn package_arg(arguments: &Value) -> Result<String, Failure> {
    let package = arguments["packageName"].as_str().unwrap_or_default();
    if resolve::valid_package(package) {
        Ok(package.to_string())
    } else {
        Err(Failure::new(
            "invalid_request",
            "packageName is not a valid Android package name",
        ))
    }
}

/// Storage client, selected account, and its bucket, or the failure envelope.
async fn setup(
    ctx: &InstallsContext,
    recorder: &Recorder,
    arguments: &Value,
    package: &str,
) -> Result<(Arc<dyn Storage>, AccountConfig, Resolution, Vec<Value>), Value> {
    let account = ctx
        .reports
        .select(arguments["account"].as_str())
        .map_err(failure)?
        .clone();
    let storage = ctx.storage().await.map_err(failure)?;
    let (resolution, candidates) = ctx
        .resolve(storage.as_ref(), recorder, &account, package)
        .await;
    match resolution {
        Ok(resolution) => Ok((storage, account, resolution, candidates)),
        Err(error) => {
            let mut envelope = failure(error);
            envelope["discovery"] = json!({"candidates": candidates});
            Err(envelope)
        }
    }
}

async fn access_check(ctx: &InstallsContext, recorder: &Recorder, arguments: &Value) -> Value {
    let package = match package_arg(arguments) {
        Ok(package) => package,
        Err(error) => return failure(error),
    };
    let override_bucket = match arguments["bucket"].as_str().map(resolve::normalize_bucket) {
        Some(Ok(bucket)) => Some(bucket),
        Some(Err(error)) => return failure(error),
        None => None,
    };
    let account = match (
        ctx.reports.select(arguments["account"].as_str()),
        &override_bucket,
    ) {
        (Ok(account), _) => account.clone(),
        (Err(_), Some(_)) => AccountConfig::default(),
        (Err(error), None) => return failure(error),
    };
    let storage = match ctx.storage().await {
        Ok(storage) => storage,
        Err(error) => return failure(error),
    };
    // Always probe fresh: this tool is the diagnostic, not a cache reader.
    let outcome = resolve::probe(
        storage.as_ref(),
        recorder,
        resolve::candidates(&account, override_bucket.as_deref()),
        &package,
    )
    .await;
    let discovery = json!({
        "account": account.developer_id,
        "principal": storage.principal(),
        "candidates": outcome.candidates,
        "configurationProblems": ctx.reports.problems,
    });
    match outcome.result {
        Ok(resolution) => {
            ctx.remember(&account, storage.as_ref(), &package, &resolution);
            let relevant = resolution.relevant == Some(true);
            json!({
                "status": if relevant { "ok" } else { "no_data" },
                "access": if relevant { "relevant_reports" } else { "accessible_empty" },
                "packageName": package,
                "resolution": resolution.to_json(),
                "discovery": discovery,
                "summary": if relevant {
                    "Install reports for this package are listable in the resolved bucket."
                } else {
                    "The bucket is accessible but holds no install reports for this package; access is proven, package relevance is not."
                },
            })
        }
        Err(error) => {
            let mut envelope = failure(error);
            envelope["packageName"] = json!(package);
            envelope["discovery"] = discovery;
            envelope
        }
    }
}

fn report_id(bucket: &str, meta: &ObjectMeta) -> String {
    let payload = json!({"b": bucket, "o": meta.name, "g": meta.generation}).to_string();
    format!(
        "{REPORT_ID_PREFIX}{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(payload)
    )
}

/// Decodes an opaque report id. The id is untrusted input: callers must still
/// check the bucket against the account's resolved bucket.
fn parse_report_id(id: &str) -> Option<(String, String, String)> {
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(id.strip_prefix(REPORT_ID_PREFIX)?)
        .ok()?;
    let value: Value = serde_json::from_slice(&bytes).ok()?;
    let text = |key: &str| value[key].as_str().map(str::to_string);
    let generation = text("g")?;
    if generation.is_empty() || !generation.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    Some((text("b")?, text("o")?, generation))
}

/// Splits `stats/installs/installs_<package>_<yyyyMM>_<dimension>.csv`. Package
/// names may contain `_` and digits, dimensions contain neither digits nor
/// anything but `[a-z_]`, so the rightmost `_<6 digits>_` is the month.
fn install_object(name: &str) -> Option<(String, String, String)> {
    let stem = name.strip_prefix(INSTALLS_PREFIX)?.strip_suffix(".csv")?;
    let bytes = stem.as_bytes();
    (0..bytes.len().saturating_sub(8)).rev().find_map(|at| {
        let month = stem.get(at + 1..at + 7)?;
        let dimension = stem.get(at + 8..)?;
        let package = stem.get(..at)?;
        (bytes[at] == b'_'
            && bytes[at + 7] == b'_'
            && month.bytes().all(|byte| byte.is_ascii_digit())
            && !dimension.is_empty()
            && dimension
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte == b'_')
            && resolve::valid_package(package))
        .then(|| {
            (
                package.to_string(),
                format!("{}-{}", &month[..4], &month[4..]),
                dimension.to_string(),
            )
        })
    })
}

fn descriptor(bucket: &str, meta: &ObjectMeta) -> Value {
    let (_, month, dimension) = install_object(&meta.name).unwrap_or_default();
    json!({
        "reportId": report_id(bucket, meta),
        "objectName": meta.name,
        "objectUri": format!("gs://{bucket}/{}", meta.name),
        "month": month,
        "dimension": dimension,
        "size": meta.size,
        "generation": meta.generation,
        "updated": meta.updated,
        "md5Hash": meta.md5_hash,
        "crc32c": meta.crc32c,
        "contentEncoding": meta.content_encoding,
    })
}

async fn list(ctx: &InstallsContext, recorder: &Recorder, arguments: &Value) -> Value {
    let package = match package_arg(arguments) {
        Ok(package) => package,
        Err(error) => return failure(error),
    };
    let (storage, account, resolution, candidates) =
        match setup(ctx, recorder, arguments, &package).await {
            Ok(setup) => setup,
            Err(envelope) => return envelope,
        };
    let limit = arguments["limit"].as_u64().unwrap_or(50).clamp(1, 100) as u32;
    let prefix = format!("{INSTALLS_PREFIX}{package}_");
    let page_token = arguments["pageToken"].as_str();
    let start_offset = arguments["fromMonth"]
        .as_str()
        .map(|month| format!("{prefix}{}", month.replace('-', "")));
    let end_offset = arguments["toMonth"]
        .as_str()
        .and_then(next_month)
        .map(|month| format!("{prefix}{month}"));
    let offsets = (start_offset.as_deref(), end_offset.as_deref());
    let page = match recorder
        .list(
            storage.as_ref(),
            &resolution.bucket,
            &prefix,
            offsets,
            page_token,
            limit,
        )
        .await
    {
        Ok(page) => page,
        Err(error) => {
            ctx.evict(&account, storage.as_ref(), &package, &error);
            return failure(&error);
        }
    };
    let dimension = arguments["dimension"].as_str();
    let reports: Vec<Value> = page
        .items
        .iter()
        .filter(|meta| {
            // The month range is already bounded by Storage offsets.
            install_object(&meta.name).is_some_and(|(object_package, _, object_dimension)| {
                object_package == package
                    && dimension.is_none_or(|wanted| wanted == object_dimension)
            })
        })
        .map(|meta| descriptor(&resolution.bucket, meta))
        .collect();
    json!({
        "status": if reports.is_empty() { "no_data" } else { "ok" },
        "packageName": package,
        "reports": reports,
        "nextPageToken": page.next_page_token,
        "note": "Filters apply within each listed page, so a page can be empty while nextPageToken is set.",
        "resolution": resolution.to_json(),
        "discovery": {"candidates": candidates},
    })
}

async fn get_raw(ctx: &InstallsContext, recorder: &Recorder, arguments: &Value) -> Reply {
    let report_id = arguments["reportId"].as_str().unwrap_or_default();
    let Some((bucket, object, generation)) = parse_report_id(report_id) else {
        return failure(Failure::new(
            "invalid_report_id",
            "reportId is not a report id issued by these tools",
        ))
        .into();
    };
    let Some((package, _, _)) = install_object(&object) else {
        return failure(Failure::new(
            "out_of_scope",
            "Only install-report objects (stats/installs/installs_*.csv) can be retrieved.",
        ))
        .into();
    };
    let (storage, account, resolution, _) = match setup(ctx, recorder, arguments, &package).await {
        Ok(setup) => setup,
        Err(envelope) => return envelope.into(),
    };
    if resolution.bucket != bucket {
        return failure(Failure::new(
            "account_mismatch",
            "The report id belongs to a different bucket than this account's report bucket.",
        ))
        .into();
    }
    let meta = match recorder.metadata(storage.as_ref(), &bucket, &object).await {
        Ok(meta) => meta,
        Err(StorageError::ObjectNotFound(_)) => {
            return source_changed(&object, &generation, None).into()
        }
        Err(error) => {
            ctx.evict(&account, storage.as_ref(), &package, &error);
            return failure(&error).into();
        }
    };
    if meta.generation != generation {
        return source_changed(&object, &generation, Some(&meta.generation)).into();
    }
    let original = arguments["mode"].as_str() == Some("original_csv");
    let limit = if original {
        MAX_ORIGINAL_BYTES
    } else {
        MAX_PARSE_BYTES
    };
    let bytes = match ctx.download(storage.as_ref(), recorder, &bucket, &meta, limit).await {
        Ok(bytes) => bytes,
        Err(StorageError::ObjectNotFound(_)) => return source_changed(&object, &generation, None).into(),
        Err(error @ StorageError::TooLarge { .. }) => {
            return failure(
                Failure::new(if original { "too_large_for_host" } else { "too_large" }, error.message())
                    .remedy("This server returns exports inline only up to the limit; no public objects or signed URLs are created."),
            )
            .into()
        }
        Err(error) => return failure(&error).into(),
    };
    let mut source = descriptor(&bucket, &meta);
    source["packageName"] = json!(package);
    let decoded = parse::decode(&bytes);
    let encoding = decoded.as_ref().map(|(_, encoding)| *encoding).ok();
    if original {
        return Reply {
            value: json!({
                "status": "ok",
                "mode": "original_csv",
                "source": source,
                "contentType": "text/csv",
                "encoding": encoding.unwrap_or("undetected"),
                "byteSize": bytes.len(),
                "sha256": format!("{:x}", Sha256::digest(bytes.as_slice())),
                "checksumScope": if meta.content_encoding.as_deref() == Some("gzip") {
                    "Google stores this export gzip-compressed: source size, md5Hash, and crc32c describe the stored gzip bytes. The attached bytes are the decompressed CSV and sha256 covers exactly them."
                } else {
                    "source md5Hash/crc32c and sha256 all describe the attached bytes."
                },
                "delivery": "The original bytes are attached unmodified as an embedded text/csv blob resource in this result.",
            }),
            original: Some(bytes.to_vec()),
        };
    }
    let records = match decoded.and_then(|(text, _)| parse::records(&text)) {
        Ok(records) => records,
        Err(message) => {
            let mut envelope = failure(
                Failure::new("parse_failed", message)
                    .remedy("Use mode original_csv to retrieve the unmodified bytes."),
            );
            envelope["source"] = source;
            return envelope.into();
        }
    };
    let offset = arguments["offset"].as_u64().unwrap_or(0) as usize;
    let page_size = (arguments["limit"].as_u64().unwrap_or(200) as usize).clamp(1, MAX_RAW_ROWS);
    let mut records = records.into_iter();
    let headers = records.next().unwrap_or_default();
    let data: Vec<Vec<String>> = records.collect();
    let rows: Vec<Value> = data
        .iter()
        .enumerate()
        .skip(offset)
        .take(page_size)
        .map(|(index, cells)| json!({"rowNumber": index + 1, "cells": cells}))
        .collect();
    let next = offset + rows.len();
    json!({
        "status": "ok",
        "mode": "rows",
        "label": "parsed_source_rows",
        "note": "Cells are the source strings in source order: not normalized, aggregated, or blank-filled. rowNumber counts data rows after the header.",
        "source": source,
        "encoding": encoding,
        "headers": headers,
        "rows": rows,
        "totalRows": data.len(),
        "nextOffset": (next < data.len()).then_some(next),
    })
    .into()
}

/// `2026-12` -> `202701`: the exclusive end offset for an inclusive `toMonth`.
fn next_month(month: &str) -> Option<String> {
    let (year, month) = month.split_once('-')?;
    let (year, month): (u32, u32) = (year.parse().ok()?, month.parse().ok()?);
    Some(if month >= 12 {
        format!("{}01", year + 1)
    } else {
        format!("{year}{:02}", month + 1)
    })
}

fn source_changed(object: &str, generation: &str, current: Option<&str>) -> Value {
    let mut envelope = failure(
        Failure::new(
            "source_changed",
            "The export no longer exists at the generation this report id pins; pages are never mixed across revisions.",
        )
        .remedy("List the reports again and use the new report id."),
    );
    envelope["objectName"] = json!(object);
    envelope["requestedGeneration"] = json!(generation);
    envelope["currentGeneration"] = json!(current);
    envelope
}

#[cfg(test)]
mod tests;
