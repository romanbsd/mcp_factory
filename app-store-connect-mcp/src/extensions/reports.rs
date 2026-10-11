//! Streaming, bounded analytics queries. Artifacts live until eviction or server exit.
use super::{check_transfer_url, failure, services::Asc, success, transfer_client};
use mcp_factory_core::{
    async_trait, CustomToolHandler, CustomToolSpec, OperationInvoker, ProxyError, ToolHints,
    ToolResult,
};
use md5::{Digest, Md5};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, VecDeque},
    io::{Read, SeekFrom},
    sync::{Arc, Mutex},
};
use tokio::{
    io::{AsyncSeekExt, AsyncWriteExt},
    sync::Semaphore,
};
const MAX_BYTES: u64 = mcp_factory_core::TransferLimits::REPORT_BYTES;
const MAX_RECORD: usize = 1024 * 1024;
const OUTPUT_BYTES: usize = 20_000;
const MAX_SEGMENTS: usize = 20;

pub(super) struct ReportService {
    pub(super) http: Result<reqwest::Client, String>,
    pub(super) transfers: Arc<Semaphore>,
    artifacts: Mutex<VecDeque<tempfile::NamedTempFile>>,
}
impl ReportService {
    pub(super) fn new(
        http: Result<reqwest::Client, String>,
        transfers: Arc<Semaphore>,
    ) -> Arc<Self> {
        Arc::new(Self {
            http,
            transfers,
            artifacts: Mutex::new(VecDeque::new()),
        })
    }
    /// Caller holds the shared permit across download and blocking consumption.
    pub(super) async fn download(&self, attributes: &Value) -> Result<std::fs::File, ToolResult> {
        let url = check_transfer_url(attributes["url"].as_str().unwrap_or_default())
            .map_err(|e| failure("lookup", &e, Value::Null))?;
        let response = transfer_client(&self.http, "download")?
            .get(url)
            .send()
            .await
            .map_err(|e| failure("download", &e.to_string(), Value::Null))?;
        self.download_response(response, attributes["checksum"].as_str())
            .await
    }
    async fn download_response(
        &self,
        mut response: reqwest::Response,
        checksum: Option<&str>,
    ) -> Result<std::fs::File, ToolResult> {
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let prefix = mcp_factory_core::read_error_prefix(
                response,
                mcp_factory_core::TransferLimits::ERROR_PREFIX_BYTES,
            )
            .await
            .map_err(|e| failure("download", &e.to_string(), Value::Null))?;
            let mut details = json!({"status":status,"upstreamBody":prefix.text,"bodyTruncated":prefix.truncated});
            if let Some(error) = prefix.read_error {
                details["bodyReadError"] = json!(error);
            }
            return Err(failure(
                "download",
                &format!("report download returned HTTP {status}"),
                details,
            ));
        }
        if response.content_length().is_some_and(|n| n > MAX_BYTES) {
            return Err(failure(
                "download",
                "segment exceeds the size limit",
                Value::Null,
            ));
        }
        let file = tempfile::tempfile().map_err(io_error)?;
        let mut file = tokio::fs::File::from_std(file);
        let mut size = 0u64;
        let mut digest = Md5::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|e| failure("download", &e.to_string(), Value::Null))?
        {
            size += chunk.len() as u64;
            if size > MAX_BYTES {
                return Err(failure(
                    "download",
                    "segment exceeds the size limit",
                    Value::Null,
                ));
            }
            digest.update(&chunk);
            file.write_all(&chunk).await.map_err(io_error)?;
        }
        let actual = digest
            .finalize()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>();
        if let Some(expected) = checksum {
            if !expected.eq_ignore_ascii_case(&actual) {
                return Err(failure(
                    "verify",
                    "checksum mismatch",
                    json!({"expected":expected,"actual":actual}),
                ));
            }
        }
        file.seek(SeekFrom::Start(0)).await.map_err(io_error)?;
        Ok(file.into_std().await)
    }
    pub(super) async fn query(
        &self,
        invoker: &dyn OperationInvoker,
        args: &Value,
    ) -> Result<Value, ToolResult> {
        let options = Options::parse(args)?;
        let client = Asc { invoker };
        let mut segments = Vec::new();
        let report = args.get("report");
        if report.is_some() && (args.get("segmentId").is_some() || args.get("reportId").is_some()) {
            return Err(failure(
                "validate",
                "provide one report source: report, segmentId or reportId",
                Value::Null,
            ));
        }
        let mut state = QueryState::new(options)?;
        if let Some(report) = report {
            let report = report.as_str().ok_or_else(|| {
                failure("validate", "report must be sales or finance", Value::Null)
            })?;
            if !["sales", "finance"].contains(&report) {
                return Err(failure(
                    "validate",
                    "report must be sales or finance",
                    Value::Null,
                ));
            }
            if args.get("dateRange").is_some() {
                return Err(failure("validate","sales/finance require reportDate for an actual report period; dateRange only selects analytics instance processingDate",Value::Null));
            }
            let vendor = super::services::required(args, "vendorNumber")?;
            let date = super::services::required(args, "reportDate")?;
            let mut query = json!({"filter[vendorNumber]":[vendor],"filter[reportDate]":[date],"filter[reportType]":[args["reportType"].as_str().unwrap_or(if report=="sales"{"SALES"}else{"FINANCIAL"})]});
            let operation = if report == "sales" {
                query["filter[frequency]"] = json!([args["frequency"].as_str().unwrap_or("DAILY")]);
                query["filter[reportSubType]"] =
                    json!([args["reportSubType"].as_str().unwrap_or("SUMMARY")]);
                if let Some(version) = args.get("reportVersion") {
                    query["filter[version]"] = json!([version]);
                }
                "salesReports_getCollection"
            } else {
                if args.get("frequency").is_some()
                    || args.get("reportSubType").is_some()
                    || args.get("reportVersion").is_some()
                {
                    return Err(failure(
                        "validate",
                        "frequency, reportSubType and reportVersion apply only to sales reports",
                        Value::Null,
                    ));
                }
                query["filter[regionCode]"] =
                    json!([super::services::required(args, "regionCode")?]);
                "financeReports_getCollection"
            };
            let permit = self
                .transfers
                .clone()
                .acquire_owned()
                .await
                .map_err(|e| failure("download", &e.to_string(), Value::Null))?;
            let response = invoker
                .stream_read_only(operation, query)
                .await
                .map_err(|e| super::proxy_error_result("download", e))?;
            let file = self.download_response(response, None).await?;
            state = blocking_with_permit(permit, move || {
                state.consume(file)?;
                Ok(state)
            })
            .await?;
        } else {
            match (args["segmentId"].as_str(), args["reportId"].as_str()) {
                (Some(id), None) if !id.is_empty() => segments.push(id.to_owned()),
                (None, Some(id)) if !id.is_empty() => {
                    let instances = client
                        .pages(
                            "analyticsReports_instances_getToManyRelated",
                            json!({"id":id,"limit":200}),
                        )
                        .await?;
                    for instance in instances {
                        if let Some(range) = args.get("dateRange") {
                            let date = instance["attributes"]["processingDate"]
                                .as_str()
                                .ok_or_else(|| {
                                    failure("lookup", "instance has no processingDate", Value::Null)
                                })?;
                            if date < range["from"].as_str().unwrap_or("")
                                || date > range["to"].as_str().unwrap_or("9999-12-31")
                            {
                                continue;
                            }
                        }
                        let rows = client
                            .pages(
                                "analyticsReportInstances_segments_getToManyRelated",
                                json!({"id":instance["id"],"limit":200}),
                            )
                            .await?;
                        for row in rows {
                            let id = row["id"].as_str().ok_or_else(|| {
                                failure("lookup", "segment has no id", Value::Null)
                            })?;
                            segments.push(id.to_owned());
                        }
                        if segments.len() > MAX_SEGMENTS {
                            return Err(failure(
                                "lookup",
                                "report exceeds 20 segments; use a narrower dateRange",
                                Value::Null,
                            ));
                        }
                    }
                }
                _ => {
                    return Err(failure(
                        "validate",
                        "provide exactly one of segmentId or reportId",
                        Value::Null,
                    ))
                }
            }
            for id in &segments {
                let permit = self
                    .transfers
                    .clone()
                    .acquire_owned()
                    .await
                    .map_err(|e| failure("download", &e.to_string(), Value::Null))?;
                let segment = client
                    .get("analyticsReportSegments_getInstance", json!({"id":id}))
                    .await?;
                let file = self.download(&segment["data"]["attributes"]).await?;
                state = blocking_with_permit(permit, move || {
                    state.consume(file)?;
                    Ok::<_, ToolResult>(state)
                })
                .await?;
            }
        }
        let mut result = state.result();
        if let Some(report) = report {
            result["report"] = report.clone();
        } else {
            result["segmentCount"] = json!(segments.len());
        }
        if let Some(mut artifact) = state.artifact.take() {
            use std::io::Write;
            artifact.flush().map_err(io_error)?;
            result["artifact"] = json!({"path":artifact.path().to_string_lossy(),"format":"csv","retention":"Last 8 artifacts retained until server exit; oldest path is invalidated on eviction."});
            let mut store = self
                .artifacts
                .lock()
                .map_err(|_| failure("artifact", "artifact store unavailable", Value::Null))?;
            if store.len() == 8 {
                store.pop_front();
            }
            store.push_back(artifact);
        }
        if super::output_budget::json_bytes(&result) > OUTPUT_BYTES {
            return Err(failure(
                "query",
                "summary exceeds output limit; select fewer columns, metrics or groups",
                Value::Null,
            ));
        }
        Ok(result)
    }
}
fn io_error(e: std::io::Error) -> ToolResult {
    failure("query", &e.to_string(), Value::Null)
}
pub(super) fn tool(service: Arc<ReportService>) -> CustomToolSpec {
    CustomToolSpec {name:"report_query".into(),description:"Query analytics segments or sales/finance report-period CSV/TSV with exact filters, projected rows, finite numeric sums and optional grouping. At most 20 segments, 100 groups, 200 rows and 20KB output. Artifact exports all matching projected rows locally; last 8 retained until eviction/server exit. dateRange selects analytics instance processingDate; sales/finance use reportDate. Numeric sums are finite f64 without financial rounding or semantic inference.".into(),
        input_schema:json!({"type":"object","properties":{
            "segmentId":{"type":"string","minLength":1},"reportId":{"type":"string","minLength":1},
            "report":{"enum":["sales","finance"]},"vendorNumber":{"type":"string","minLength":1},"reportDate":{"type":"string","minLength":1},"frequency":{"type":"string"},"reportType":{"type":"string"},"reportSubType":{"type":"string"},"regionCode":{"type":"string","minLength":1},"reportVersion":{"type":"string"},
            "dateRange":{"type":"object","properties":{"from":{"type":"string","pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"},"to":{"type":"string","pattern":"^[0-9]{4}-[0-9]{2}-[0-9]{2}$"}},"required":["from","to"],"additionalProperties":false},
            "columns":{"type":"array","items":{"type":"string"},"maxItems":100,"uniqueItems":true},
            "filters":{"type":"object","additionalProperties":{"type":["string","number","boolean","null"]},"maxProperties":100},
            "groupBy":{"type":"array","items":{"type":"string"},"maxItems":10,"uniqueItems":true},
            "metrics":{"type":"array","items":{"type":"string"},"maxItems":100,"uniqueItems":true},
            "limit":{"type":"integer","minimum":1,"maximum":200},"output":{"enum":["summary","rows","artifact"]}},
            "oneOf":[{"required":["segmentId"],"not":{"anyOf":[{"required":["reportId"]},{"required":["report"]}]}},{"required":["reportId"],"not":{"anyOf":[{"required":["segmentId"]},{"required":["report"]}]}},{"required":["report","vendorNumber","reportDate"],"not":{"anyOf":[{"required":["segmentId"]},{"required":["reportId"]}]}}],"additionalProperties":false}),
        hints:ToolHints{title:Some("Query analytics report".into()),read_only:Some(true),destructive:Some(false),idempotent:Some(true),open_world:Some(true),..Default::default()},handler:Arc::new(QueryHandler(service))}
}
struct QueryHandler(Arc<ReportService>);
#[async_trait]
impl CustomToolHandler for QueryHandler {
    async fn call(
        &self,
        invoker: &dyn OperationInvoker,
        args: Value,
    ) -> Result<ToolResult, ProxyError> {
        Ok(match self.0.query(invoker, &args).await {
            Ok(v) => success(v),
            Err(e) => e,
        })
    }
}
#[derive(Clone)]
struct Options {
    columns: Vec<String>,
    filters: BTreeMap<String, String>,
    groups: Vec<String>,
    metrics: Vec<String>,
    limit: usize,
    output: String,
}
impl Options {
    fn parse(args: &Value) -> Result<Self, ToolResult> {
        if args
            .get("filters")
            .is_some_and(|v| !v.is_object() || v.as_object().is_some_and(|m| m.len() > 100))
        {
            return Err(failure(
                "validate",
                "filters must be an object with at most 100 entries",
                Value::Null,
            ));
        }
        fn names(args: &Value, key: &str) -> Result<Vec<String>, ToolResult> {
            match args.get(key) {
                None => Ok(Vec::new()),
                Some(Value::Array(a))
                    if a.len() <= if key == "groupBy" {10} else {100}
                        && a.iter()
                            .map(Value::to_string)
                            .collect::<std::collections::HashSet<_>>()
                            .len()
                            == a.len() =>
                {
                    a.iter()
                        .map(|v| {
                            v.as_str()
                                .filter(|s| !s.is_empty())
                                .map(str::to_owned)
                                .ok_or_else(|| {
                                    failure(
                                        "validate",
                                        "column names must be nonempty strings",
                                        Value::Null,
                                    )
                                })
                        })
                        .collect()
                }
                _ => Err(failure(
                    "validate",
                    "column selections must be unique bounded arrays (100 columns/metrics, 10 groupBy)",
                    Value::Null,
                )),
            }
        }
        let filters = args
            .get("filters")
            .and_then(Value::as_object)
            .map(|m| {
                m.iter()
                    .map(|(k, v)| {
                        if v.is_object() || v.is_array() {
                            Err(failure(
                                "validate",
                                "filters must be exact scalar values",
                                Value::Null,
                            ))
                        } else {
                            Ok((
                                k.clone(),
                                v.as_str()
                                    .map(str::to_owned)
                                    .unwrap_or_else(|| v.to_string()),
                            ))
                        }
                    })
                    .collect()
            })
            .transpose()?
            .unwrap_or_default();
        let output = match args.get("output") {
            None => "summary",
            Some(v) => v
                .as_str()
                .ok_or_else(|| failure("validate", "output must be a string", Value::Null))?,
        };
        if !["summary", "rows", "artifact"].contains(&output) {
            return Err(failure("validate", "unknown output", Value::Null));
        }
        if let Some(range) = args.get("dateRange") {
            let from = range["from"].as_str().unwrap_or("");
            let to = range["to"].as_str().unwrap_or("");
            if !valid_date(from) || !valid_date(to) || from > to {
                return Err(failure(
                    "validate",
                    "dateRange requires ordered YYYY-MM-DD dates",
                    Value::Null,
                ));
            }
        }
        let limit = match args.get("limit") {
            None => 20,
            Some(v) => v
                .as_u64()
                .ok_or_else(|| failure("validate", "limit must be an integer", Value::Null))?,
        };
        let limit = usize::try_from(limit).unwrap_or(usize::MAX);
        if !(1..=200).contains(&limit) {
            return Err(failure("validate", "limit must be 1..200", Value::Null));
        }
        Ok(Self {
            columns: names(args, "columns")?,
            filters,
            groups: names(args, "groupBy")?,
            metrics: names(args, "metrics")?,
            limit,
            output: output.into(),
        })
    }
}
struct QueryState {
    options: Options,
    headers: Vec<String>,
    row_count: u64,
    rows: Vec<Value>,
    row_budget: super::output_budget::ArrayBudget,
    sums: Vec<f64>,
    groups: BTreeMap<Vec<String>, (u64, Vec<f64>)>,
    artifact: Option<tempfile::NamedTempFile>,
    decoded: u64,
}
impl QueryState {
    fn new(options: Options) -> Result<Self, ToolResult> {
        let artifact = if options.output == "artifact" {
            Some(tempfile::NamedTempFile::new().map_err(io_error)?)
        } else {
            None
        };
        Ok(Self {
            sums: vec![0.; options.metrics.len()],
            options,
            headers: Vec::new(),
            row_count: 0,
            rows: Vec::new(),
            row_budget: super::output_budget::ArrayBudget::new(9_999),
            groups: BTreeMap::new(),
            artifact,
            decoded: 0,
        })
    }
    fn consume(&mut self, file: std::fs::File) -> Result<(), ToolResult> {
        let reader = mcp_factory_core::decoded_reader(file).map_err(io_error)?;
        let mut reader = BoundedRead {
            inner: reader,
            bytes: 0,
            max: MAX_BYTES - self.decoded,
            record: 0,
            quoted: false,
        };
        // Probe just the first bounded physical header line. CSV quoted multiline headers are rejected.
        let mut header = Vec::new();
        loop {
            let mut b = [0];
            if reader.read(&mut b).map_err(io_error)? == 0 {
                break;
            }
            header.push(b[0]);
            if b[0] == b'\n' {
                break;
            }
        }
        if header.is_empty() {
            return Err(failure("query", "report has no header", Value::Null));
        }
        let mut quoted = false;
        let has_tab = header.iter().any(|b| {
            if *b == b'"' {
                quoted = !quoted;
            }
            *b == b'\t' && !quoted
        });
        let delimiter = if has_tab { b'\t' } else { b',' };
        let chain = std::io::Cursor::new(header).chain(&mut reader);
        let mut csv = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .from_reader(chain);
        let headers = csv
            .headers()
            .map_err(csv_error)?
            .iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if headers.len() > 100
            || headers.iter().any(|s| s.len() > 256)
            || headers
                .iter()
                .collect::<std::collections::HashSet<_>>()
                .len()
                != headers.len()
        {
            return Err(failure(
                "query",
                "header exceeds column limits or has duplicate names",
                Value::Null,
            ));
        }
        if self.headers.is_empty() {
            self.headers = headers.clone();
        } else if self.headers != headers {
            return Err(failure("query", "segment headers differ", Value::Null));
        }
        let index = |name: &String| {
            headers
                .iter()
                .position(|h| h == name)
                .ok_or_else(|| failure("query", &format!("unknown column: {name}"), Value::Null))
        };
        let projected = if self.options.columns.is_empty() {
            headers.clone()
        } else {
            self.options.columns.clone()
        };
        let projection = projected.iter().map(index).collect::<Result<Vec<_>, _>>()?;
        let metrics = self
            .options
            .metrics
            .iter()
            .map(index)
            .collect::<Result<Vec<_>, _>>()?;
        let groups = self
            .options
            .groups
            .iter()
            .map(index)
            .collect::<Result<Vec<_>, _>>()?;
        let filters = self
            .options
            .filters
            .iter()
            .map(|(k, v)| Ok((index(k)?, v.clone())))
            .collect::<Result<Vec<_>, ToolResult>>()?;
        let mut writer = self.artifact.as_mut().map(|f| {
            let bytes = f.as_file().metadata().map(|m| m.len()).unwrap_or(MAX_BYTES);
            csv::Writer::from_writer(BoundedWrite {
                inner: f.as_file_mut(),
                bytes,
            })
        });
        if self.decoded == 0 {
            if let Some(w) = writer.as_mut() {
                w.write_record(&projected).map_err(csv_error)?;
            }
        }
        for record in csv.records() {
            let record = record.map_err(csv_error)?;
            if record.iter().any(|s| s.len() > 64 * 1024) {
                return Err(failure("query", "cell exceeds 64KiB limit", Value::Null));
            }
            if filters
                .iter()
                .any(|(i, v)| record.get(*i) != Some(v.as_str()))
            {
                continue;
            }
            self.row_count += 1;
            let values = metrics
                .iter()
                .map(|i| {
                    record[*i]
                        .parse::<f64>()
                        .ok()
                        .filter(|v| v.is_finite())
                        .ok_or_else(|| {
                            failure(
                                "query",
                                &format!("invalid finite numeric metric: {}", headers[*i]),
                                Value::Null,
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            for (sum, v) in self.sums.iter_mut().zip(&values) {
                *sum += v;
                if !sum.is_finite() {
                    return Err(failure("query", "numeric sum overflow", Value::Null));
                }
            }
            if !groups.is_empty() {
                let key = groups
                    .iter()
                    .map(|i| record[*i].to_owned())
                    .collect::<Vec<_>>();
                if !self.groups.contains_key(&key) && self.groups.len() == 100 {
                    return Err(failure(
                        "query",
                        "group limit exceeded (100); narrow filters",
                        Value::Null,
                    ));
                }
                let entry = self
                    .groups
                    .entry(key)
                    .or_insert_with(|| (0, vec![0.; values.len()]));
                entry.0 += 1;
                for (sum, v) in entry.1.iter_mut().zip(&values) {
                    *sum += v;
                    if !sum.is_finite() {
                        return Err(failure("query", "group sum overflow", Value::Null));
                    }
                }
            }
            if let Some(w) = writer.as_mut() {
                w.write_record(projection.iter().map(|i| &record[*i]))
                    .map_err(csv_error)?;
            }
            if self.options.output == "rows" && self.rows.len() < self.options.limit {
                let row = Value::Object(
                    projected
                        .iter()
                        .zip(&projection)
                        .map(|(name, i)| (name.clone(), json!(&record[*i])))
                        .collect(),
                );
                // Retain whole rows only: no silent cell truncation and no unbounded accumulated output.
                if self.row_budget.push(&row) {
                    self.rows.push(row);
                }
            }
        }
        if let Some(w) = writer.as_mut() {
            w.flush().map_err(io_error)?;
        }
        drop(writer);
        drop(csv);
        self.decoded += reader.bytes;
        Ok(())
    }
    fn result(&self) -> Value {
        let sums = |v: &[f64]| -> Value {
            Value::Object(
                self.options
                    .metrics
                    .iter()
                    .cloned()
                    .zip(v.iter().map(|n| json!(n)))
                    .collect(),
            )
        };
        let mut value = json!({"ok":true,"rowCount":self.row_count,"columns":if self.options.columns.is_empty(){&self.headers}else{&self.options.columns},"sums":sums(&self.sums),"decodedBytes":self.decoded});
        if !self.options.groups.is_empty() {
            value["groups"]=json!(self.groups.iter().map(|(key,(count,sum))|json!({"key":self.options.groups.iter().cloned().zip(key.iter().cloned()).collect::<BTreeMap<_,_>>(),"rowCount":count,"sums":sums(sum)})).collect::<Vec<_>>());
        }
        if self.options.output == "rows" {
            value["rows"] = json!(self.rows);
            value["droppedRows"] = json!(self.row_count - self.rows.len() as u64);
            value["truncated"] = json!(self.row_count > self.rows.len() as u64);
        }
        value
    }
}
fn csv_error(e: csv::Error) -> ToolResult {
    failure("query", &e.to_string(), Value::Null)
}
/// Limit decoded bytes and logical CSV record bytes before csv can grow its buffer.
struct BoundedRead<R> {
    inner: R,
    bytes: u64,
    max: u64,
    record: usize,
    quoted: bool,
}
impl<R: Read> Read for BoundedRead<R> {
    fn read(&mut self, out: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(out)?;
        for b in &out[..n] {
            self.bytes += 1;
            self.record += 1;
            if self.bytes > self.max {
                return Err(std::io::Error::other(
                    "decompressed report exceeds size limit",
                ));
            }
            if self.record > MAX_RECORD {
                return Err(std::io::Error::other("CSV record exceeds 1MiB limit"));
            }
            if *b == b'"' {
                self.quoted = !self.quoted;
            }
            if *b == b'\n' && !self.quoted {
                self.record = 0;
            }
        }
        Ok(n)
    }
}
#[cfg(test)]
#[path = "report_tests.rs"]
mod tests;

/// The permit belongs to blocking work, so aborting its async waiter cannot free it early.
pub(super) async fn blocking_with_permit<T: Send + 'static>(
    permit: tokio::sync::OwnedSemaphorePermit,
    work: impl FnOnce() -> Result<T, ToolResult> + Send + 'static,
) -> Result<T, ToolResult> {
    tokio::task::spawn_blocking(move || {
        let _permit = permit;
        work()
    })
    .await
    .map_err(|e| failure("query", &e.to_string(), Value::Null))?
}

struct BoundedWrite<W> {
    inner: W,
    bytes: u64,
}
impl<W: std::io::Write> std::io::Write for BoundedWrite<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.bytes + buf.len() as u64 > MAX_BYTES {
            return Err(std::io::Error::other("artifact exceeds 256MiB limit"));
        }
        let n = self.inner.write(buf)?;
        self.bytes += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

fn valid_date(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 10
        || b[4] != b'-'
        || b[7] != b'-'
        || b.iter()
            .enumerate()
            .any(|(i, b)| i != 4 && i != 7 && !b.is_ascii_digit())
    {
        return false;
    }
    let year = s[..4].parse::<u32>().unwrap_or(0);
    let month = s[5..7].parse::<u32>().unwrap_or(0);
    let day = s[8..].parse::<u32>().unwrap_or(0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                29
            } else {
                28
            }
        }
        _ => 0,
    };
    year > 0 && day > 0 && day <= days
}
