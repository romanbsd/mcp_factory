use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use mcp_factory_core::{async_trait, ProxyError, ReadOnlyToolInvoker, ToolResult};
use serde_json::{json, Value};
use tokio::sync::Semaphore;
use wiremock::matchers::{body_bytes, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use super::asset_upload::{md5_hex, AssetUpload, KINDS};

struct Invoker {
    replies: Mutex<VecDeque<ToolResult>>,
    calls: Mutex<Vec<(String, Value)>>,
}
impl Invoker {
    fn new(replies: Vec<ToolResult>) -> Self {
        Self {
            replies: Mutex::new(replies.into()),
            calls: Mutex::new(Vec::new()),
        }
    }
    fn take(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.calls.lock().unwrap().push((name.to_owned(), args));
        Ok(self
            .replies
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected API call"))
    }
}
#[async_trait]
impl ReadOnlyToolInvoker for Invoker {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.take(name, args)
    }
    async fn invoke_mutating(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.take(name, args)
    }
}
fn reply(value: Value) -> ToolResult {
    super::success(value)
}
fn upload(root: &std::path::Path) -> AssetUpload {
    AssetUpload {
        http: Ok(reqwest::Client::new()),
        media_root: Some(root.to_owned()),
        kinds: KINDS.iter().collect(),
        poll_interval: Duration::from_millis(1),
        transfers: Arc::new(Semaphore::new(2)),
    }
}
fn args() -> Value {
    json!({"kind": "appScreenshot", "parentId": "set-1", "file": "image.png", "waitSeconds": 0})
}
fn resource(state: &str, checksum: &str, operations: Vec<Value>) -> Value {
    json!({"data": {"type": "appScreenshots", "id": "asset-1",
        "relationships": {"appScreenshotSet": {"data": {"type": "appScreenshotSets", "id": "set-1"}}},
        "attributes": {"fileName": "image.png", "fileSize": 10,
            "sourceFileChecksum": checksum, "assetDeliveryState": {"state": state}, "uploadOperations": operations}}})
}
fn operation(url: &str, offset: u64, length: u64) -> Value {
    json!({"url": url, "method": "PUT", "offset": offset, "length": length,
        "requestHeaders": [{"name": "Content-Length", "value": "99999"}]})
}
async fn fixture() -> (tempfile::TempDir, String) {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("image.png"), b"0123456789").unwrap();
    let checksum = md5_hex(&root.path().join("image.png")).await.unwrap();
    (root, checksum)
}
fn resume(checksum: &str) -> Value {
    json!({"assetId": "asset-1", "fileSize": 10, "sourceFileChecksum": checksum,
        "completedRanges": [], "ranges": []})
}

#[tokio::test]
async fn nonzero_range_streams_exact_bytes_and_explicit_length() {
    let (root, checksum) = fixture().await;
    let server = MockServer::start().await;
    let url = format!("{}/range", server.uri());
    Mock::given(method("PUT"))
        .and(path("/range"))
        .and(header("content-length", "6"))
        .and(body_bytes(b"345678".to_vec()))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    let invoker = Invoker::new(vec![
        reply(resource("AWAITING_UPLOAD", "", vec![operation(&url, 3, 6)])),
        reply(resource("COMPLETE", &checksum, vec![])),
    ]);
    let result = upload(root.path()).run(&invoker, &args()).await.unwrap();
    assert_eq!(result.structured.unwrap()["stage"], "complete");
    assert_eq!(invoker.calls.lock().unwrap().len(), 2);
}

#[tokio::test]
async fn upload_failure_checkpoint_resumes_without_second_reservation() {
    let (root, checksum) = fixture().await;
    let server = MockServer::start().await;
    let first = format!("{}/first", server.uri());
    let second = format!("{}/second", server.uri());
    let operations = vec![operation(&first, 0, 5), operation(&second, 5, 5)];
    Mock::given(path("/first"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/second"))
        .respond_with(ResponseTemplate::new(500).set_body_string("part failed"))
        .expect(1)
        .mount(&server)
        .await;
    let invoker = Invoker::new(vec![reply(resource(
        "AWAITING_UPLOAD",
        "",
        operations.clone(),
    ))]);
    let service = upload(root.path());
    let result = service
        .run(&invoker, &args())
        .await
        .unwrap_err()
        .structured
        .unwrap();
    assert_eq!(result["stage"], "upload");
    assert_eq!(result["resume"]["assetId"], "asset-1");
    assert_eq!(result["resume"]["completedRanges"], json!([0]));
    assert_eq!(result["resume"]["pendingRanges"], json!([1]));
    server.verify().await;
    server.reset().await;
    Mock::given(path("/second"))
        .and(body_bytes(b"56789".to_vec()))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;
    let invoker = Invoker::new(vec![
        reply(resource("AWAITING_UPLOAD", "", operations)),
        reply(resource("COMPLETE", &checksum, vec![])),
    ]);
    let mut retry = args();
    retry["resume"] = result["resume"].clone();
    service.run(&invoker, &retry).await.unwrap();
    let calls = invoker.calls.lock().unwrap().clone();
    assert!(calls
        .iter()
        .all(|(name, _)| !name.ends_with("createInstance")));
    assert_eq!(calls[0].1["include"], json!(["appScreenshotSet"]));
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn resume_checksum_mismatch_fails_before_any_api_call() {
    let (root, _) = fixture().await;
    let invoker = Invoker::new(vec![]);
    let mut input = args();
    input["resume"] = resume("00000000000000000000000000000000");
    let result = upload(root.path()).run(&invoker, &input).await.unwrap_err();
    let error = result.structured.unwrap();
    assert_eq!(error["stage"], "validate");
    assert_eq!(error["resume"], input["resume"]);
    assert!(invoker.calls.lock().unwrap().is_empty());
}

#[tokio::test]
async fn complete_resume_only_reads_existing_asset() {
    let (root, checksum) = fixture().await;
    let invoker = Invoker::new(vec![reply(resource("COMPLETE", &checksum, vec![]))]);
    let mut input = args();
    input["resume"] = resume(&checksum);
    let result = upload(root.path()).run(&invoker, &input).await.unwrap();
    assert_eq!(result.structured.unwrap()["stage"], "complete");
    let calls = invoker.calls.lock().unwrap().clone();
    assert_eq!(calls.len(), 1);
    assert!(calls[0].0.ends_with("getInstance"));
}

#[tokio::test]
async fn failed_commit_keeps_ranges_and_processing_error_keeps_identity() {
    let (root, checksum) = fixture().await;
    let server = MockServer::start().await;
    let op = operation(&format!("{}/part", server.uri()), 0, 10);
    Mock::given(path("/part"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;
    let invoker = Invoker::new(vec![
        reply(resource("AWAITING_UPLOAD", "", vec![op.clone()])),
        super::failure(
            "upstream",
            "commit rejected",
            json!({"errors": [{"code": "TEST"}]}),
        ),
    ]);
    let result = upload(root.path())
        .run(&invoker, &args())
        .await
        .unwrap_err()
        .structured
        .unwrap();
    assert_eq!(result["stage"], "commit");
    assert_eq!(result["resume"]["completedRanges"], json!([0]));
    assert_eq!(result["resume"]["committed"], false);
    let invoker = Invoker::new(vec![
        reply(resource("AWAITING_UPLOAD", "", vec![op])),
        reply(resource("UPLOAD_COMPLETE", &checksum, vec![])),
        super::failure(
            "upstream",
            "poll rejected",
            json!({"errors": [{"code": "TEST"}]}),
        ),
    ]);
    let mut input = args();
    input["waitSeconds"] = json!(1);
    let result = upload(root.path())
        .run(&invoker, &input)
        .await
        .unwrap_err()
        .structured
        .unwrap();
    assert_eq!(result["stage"], "processing");
    assert_eq!(result["resume"]["assetId"], "asset-1");
    assert_eq!(result["resume"]["committed"], true);
    assert!(result["details"]["upstream"]["details"]["errors"].is_array());
}

#[tokio::test]
async fn changed_resume_ranges_and_wrong_parent_are_rejected_before_upload() {
    let (root, checksum) = fixture().await;
    for wrong_parent in [false, true] {
        let mut data = resource(
            "AWAITING_UPLOAD",
            "",
            vec![operation("https://unused.example/", 0, 10)],
        );
        if wrong_parent {
            data["data"]["relationships"]["appScreenshotSet"]["data"]["id"] = json!("other");
        }
        let invoker = Invoker::new(vec![reply(data)]);
        let mut input = args();
        input["resume"] = resume(&checksum);
        input["resume"]["completedRanges"] = json!([0]);
        input["resume"]["ranges"] = json!([{"offset": 1, "length": 9}]);
        let result = upload(root.path()).run(&invoker, &input).await.unwrap_err();
        assert_eq!(result.structured.unwrap()["stage"], "resume");
        assert_eq!(invoker.calls.lock().unwrap().len(), 1);
    }
}

#[tokio::test]
async fn large_file_limit_and_error_body_cap() {
    let (root, _) = fixture().await;
    std::fs::OpenOptions::new()
        .write(true)
        .open(root.path().join("image.png"))
        .unwrap()
        .set_len(2 * 1024 * 1024 * 1024 + 1)
        .unwrap();
    let invoker = Invoker::new(vec![]);
    let result = upload(root.path())
        .run(&invoker, &args())
        .await
        .unwrap_err();
    assert!(result.into_text().contains("2 GiB"));
    assert!(invoker.calls.lock().unwrap().is_empty());
    std::fs::write(root.path().join("image.png"), b"0123456789").unwrap();
    let server = MockServer::start().await;
    Mock::given(path("/bad"))
        .respond_with(
            ResponseTemplate::new(500).set_body_string(format!("useful: {}", "x".repeat(100_000))),
        )
        .mount(&server)
        .await;
    let invoker = Invoker::new(vec![reply(resource(
        "AWAITING_UPLOAD",
        "",
        vec![operation(&format!("{}/bad", server.uri()), 0, 10)],
    ))]);
    let result = upload(root.path())
        .run(&invoker, &args())
        .await
        .unwrap_err()
        .structured
        .unwrap();
    let error = result["details"]["error"].as_str().unwrap();
    assert!(error.contains("useful:"));
    assert!(error.len() <= 2100, "{}", error.len());
}

struct SlowPoll(Invoker);
#[async_trait]
impl ReadOnlyToolInvoker for SlowPoll {
    async fn invoke_read_only(&self, _: &str, _: Value) -> Result<ToolResult, ProxyError> {
        tokio::time::sleep(Duration::from_secs(30)).await;
        panic!("processing deadline did not cancel polling")
    }
    async fn invoke_mutating(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        self.0.take(name, args)
    }
}

#[tokio::test]
async fn processing_deadline_bounds_a_slow_api_poll() {
    let (root, checksum) = fixture().await;
    let server = MockServer::start().await;
    Mock::given(path("/part"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;
    let invoker = SlowPoll(Invoker::new(vec![
        reply(resource(
            "AWAITING_UPLOAD",
            "",
            vec![operation(&format!("{}/part", server.uri()), 0, 10)],
        )),
        reply(resource("PROCESSING", &checksum, vec![])),
    ]));
    let mut input = args();
    input["waitSeconds"] = json!(1);
    let started = tokio::time::Instant::now();
    let result = upload(root.path())
        .run(&invoker, &input)
        .await
        .unwrap()
        .structured
        .unwrap();
    assert!(started.elapsed() < Duration::from_secs(3));
    assert_eq!(result["stage"], "processing");
    assert_eq!(result["asset"]["state"], "PROCESSING");
    assert_eq!(result["resume"]["committed"], true);
}

#[tokio::test]
async fn large_sparse_upload_range_yields_bounded_chunks_without_retaining_them() {
    use std::io::{Seek, SeekFrom, Write};

    use futures_util::StreamExt;

    const OFFSET: u64 = 7;
    const LENGTH: u64 = 64 * 1024 * 1024;
    const BUFFER_LIMIT: usize = 64 * 1024;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("large-sparse.bin");
    let mut sparse = std::fs::File::create(&path).unwrap();
    sparse.set_len(OFFSET + LENGTH + 7).unwrap();
    sparse.write_all(&[0xff; 7]).unwrap();
    sparse.seek(SeekFrom::Start(OFFSET + LENGTH)).unwrap();
    sparse.write_all(&[0xfe; 7]).unwrap();
    drop(sparse);

    // This is the same constructor whose stream is wrapped in reqwest's upload body.
    let mut stream = super::asset_upload::upload_range_stream(&path, OFFSET, LENGTH)
        .await
        .unwrap();
    let mut total = 0u64;
    let mut chunks = 0usize;
    let mut largest = 0usize;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        assert!(!chunk.is_empty());
        assert!(
            chunk.len() <= BUFFER_LIMIT,
            "chunk has {} bytes",
            chunk.len()
        );
        // Nonzero sentinels outside the requested range must never appear.
        assert!(chunk.iter().all(|byte| *byte == 0));
        total += chunk.len() as u64;
        chunks += 1;
        largest = largest.max(chunk.len());
        // Each chunk is dropped here; the full 64 MiB part is never collected.
    }
    assert_eq!(total, LENGTH);
    assert!(chunks >= LENGTH as usize / BUFFER_LIMIT);
    assert!(largest <= BUFFER_LIMIT);
}
