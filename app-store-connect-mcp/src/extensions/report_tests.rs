use super::*;
use std::io::{Seek, Write};
fn state(args: Value, bytes: &[u8]) -> Result<QueryState, ToolResult> {
    let mut state = QueryState::new(Options::parse(&args)?)?;
    let mut file = tempfile::tempfile().unwrap();
    file.write_all(bytes).unwrap();
    file.rewind().unwrap();
    state.consume(file)?;
    Ok(state)
}
fn gzip(bytes: &[u8]) -> Vec<u8> {
    let mut z = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    z.write_all(bytes).unwrap();
    z.finish().unwrap()
}
#[test]
fn exact_filtered_grouped_sums() {
    let s = state(
        json!({"filters":{"Country":"US"},"groupBy":["Device"],"metrics":["Count"]}),
        b"Country\tDevice\tCount\nUS\tphone\t2\nGB\tphone\t99\nUS\tphone\t3.5\nUS\tpad\t1\n",
    )
    .unwrap();
    let r = s.result();
    assert_eq!(r["rowCount"], 3);
    assert_eq!(r["sums"]["Count"], 6.5);
    assert_eq!(r["groups"][1]["sums"]["Count"], 5.5);
}
#[test]
fn projection_row_limits_and_quoted_csv() {
    let r = state(
        json!({"columns":["Name"],"output":"rows","limit":1}),
        b"Name,Count\n\"a,b\",2\n\"line\nnext\",3\n",
    )
    .unwrap()
    .result();
    assert_eq!(r["rows"], json!([{"Name":"a,b"}]));
    assert_eq!(r["droppedRows"], 1);
    assert_eq!(r["truncated"], true);
    assert!(state(json!({"columns":["missing"]}), b"A\n1\n").is_err());
    assert!(state(json!({"metrics":["A"]}), b"A\nNaN\n").is_err());
    assert!(state(json!({"metrics":["A"]}), b"A\n\n").is_ok()); // CSV blank lines are not records.
    assert!(state(json!({"metrics":["A"]}), b"A,B\n,1\n").is_err());
}
#[test]
fn bounded_decode_invalid_utf8_and_large_records() {
    assert!(state(json!({}), &gzip(b"A\n\xff\n")).is_err());
    assert!(state(json!({}), &[b'A', b'\n', 0xff]).is_err());
    let mut reader = BoundedRead {
        inner: std::io::Cursor::new(vec![b'a'; 10000]),
        bytes: 0,
        max: 99,
        record: 0,
        quoted: false,
    };
    let mut buf = [0; 100];
    assert!(reader.read(&mut buf).is_err());
    let mut decoded = BoundedRead {
        inner: flate2::read::MultiGzDecoder::new(std::io::Cursor::new(gzip(&vec![b'a'; 10000]))),
        bytes: 0,
        max: 9999,
        record: 0,
        quoted: false,
    };
    assert!(std::io::copy(&mut decoded, &mut std::io::sink()).is_err());
    let mut bytes = b"A\n\"".to_vec();
    bytes.extend(vec![b'a'; MAX_RECORD]);
    assert!(state(json!({}), &bytes).is_err());
    let mut bytes = b"A\n".to_vec();
    bytes.extend(vec![b'a'; 65537]);
    assert!(state(json!({}), &bytes).is_err());
}
#[test]
fn artifact_contains_full_projected_matching_rows() {
    let mut s = state(
        json!({"columns":["A"],"filters":{"B":"yes"},"output":"artifact","limit":1}),
        b"A,B\nx,yes\ny,no\nz,yes\n",
    )
    .unwrap();
    let file = s.artifact.as_mut().unwrap();
    file.as_file_mut().rewind().unwrap();
    let mut out = String::new();
    file.read_to_string(&mut out).unwrap();
    assert_eq!(out, "A\nx\nz\n");
    assert_eq!(s.row_count, 2);
}
#[test]
fn group_cap_and_output_budget() {
    let mut csv = "A,B\n".to_owned();
    for i in 0..101 {
        csv += &format!("{i},1\n");
    }
    assert!(state(json!({"groupBy":["A"]}), csv.as_bytes()).is_err());
    let mut csv = "A\n".to_owned();
    for _ in 0..100 {
        csv += &format!("{}\n", "x".repeat(1000));
    }
    let result = state(json!({"output":"rows","limit":200}), csv.as_bytes())
        .unwrap()
        .result();
    assert!(serde_json::to_vec(&result).unwrap().len() < 20000);
    assert!(result["droppedRows"].as_u64().unwrap() > 0);
}
struct Invoker {
    url: String,
}
#[async_trait]
impl OperationInvoker for Invoker {
    async fn invoke_read_only(&self, name: &str, _args: Value) -> Result<ToolResult, ProxyError> {
        assert_eq!(name, "analyticsReportSegments_getInstance");
        Ok(success(json!({"data":{"attributes":{"url":self.url}}})))
    }
}
#[tokio::test]
async fn shared_budget_blocks_report_and_preview_and_releases_on_cancel() {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string("A\n1\n")
                .set_delay(std::time::Duration::from_millis(300)),
        )
        .mount(&server)
        .await;
    let budget = Arc::new(Semaphore::new(1));
    let service = ReportService::new(Ok(reqwest::Client::new()), budget.clone());
    let invoker = Arc::new(Invoker { url: server.uri() });
    let service1 = service.clone();
    let invoker1 = invoker.clone();
    let first = tokio::spawn(async move {
        service1
            .query(invoker1.as_ref(), &json!({"segmentId":"1"}))
            .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(1), async {
        while budget.available_permits() != 0 {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    let handler = super::super::analytics::tool_with_service(service.clone()).handler;
    let invoker2 = invoker.clone();
    let second = tokio::spawn(async move {
        handler
            .call(invoker2.as_ref(), json!({"segmentId":"2"}))
            .await
    });
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
    first.abort();
    assert!(!second.await.unwrap().unwrap().is_error);
    assert_eq!(budget.available_permits(), 1);
}
#[tokio::test]
async fn blocking_cancellation_keeps_budget_until_work_finishes() {
    let budget = Arc::new(Semaphore::new(1));
    let permit = budget.clone().acquire_owned().await.unwrap();
    let (started_tx, started_rx) = tokio::sync::oneshot::channel();
    let (finish_tx, finish_rx) = std::sync::mpsc::channel();
    let task = tokio::spawn(blocking_with_permit(permit, move || {
        started_tx.send(()).unwrap();
        finish_rx.recv().unwrap();
        Ok(())
    }));
    started_rx.await.unwrap();
    task.abort();
    assert_eq!(budget.available_permits(), 0);
    assert!(tokio::time::timeout(
        std::time::Duration::from_millis(30),
        budget.clone().acquire_owned()
    )
    .await
    .is_err());
    finish_tx.send(()).unwrap();
    let _permit = tokio::time::timeout(std::time::Duration::from_secs(1), budget.acquire())
        .await
        .unwrap()
        .unwrap();
}
struct Discovery {
    url: String,
    count: usize,
}
#[async_trait]
impl OperationInvoker for Discovery {
    async fn invoke_read_only(&self, name: &str, args: Value) -> Result<ToolResult, ProxyError> {
        let data = match name {
            "analyticsReports_instances_getToManyRelated" => {
                json!([{"id":"old","attributes":{"processingDate":"2026-01-01"}},{"id":"new","attributes":{"processingDate":"2026-10-11"}}])
            }
            "analyticsReportInstances_segments_getToManyRelated" => {
                assert_eq!(args["id"], "new");
                json!((0..self.count)
                    .map(|i| json!({"id":i.to_string()}))
                    .collect::<Vec<_>>())
            }
            "analyticsReportSegments_getInstance" => json!({"attributes":{"url":self.url}}),
            _ => panic!("unexpected {name}"),
        };
        Ok(success(json!({"data":data})))
    }
}
#[tokio::test]
async fn report_discovery_filters_processing_date_and_bounds_segments() {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("A\n2\n"))
        .mount(&server)
        .await;
    let service = ReportService::new(Ok(reqwest::Client::new()), Arc::new(Semaphore::new(2)));
    let args =
        json!({"reportId":"r","dateRange":{"from":"2026-10-01","to":"2026-10-31"},"metrics":["A"]});
    let result = service
        .query(
            &Discovery {
                url: server.uri(),
                count: 2,
            },
            &args,
        )
        .await
        .unwrap();
    assert_eq!(result["rowCount"], 2);
    assert_eq!(result["sums"]["A"], 4.0);
    assert!(service
        .query(
            &Discovery {
                url: server.uri(),
                count: 21
            },
            &args
        )
        .await
        .unwrap_err()
        .into_text()
        .contains("20 segments"));
}
#[tokio::test]
async fn ninth_artifact_evicts_oldest() {
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_string("A\n2\n"))
        .mount(&server)
        .await;
    let service = ReportService::new(Ok(reqwest::Client::new()), Arc::new(Semaphore::new(2)));
    let invoker = Invoker { url: server.uri() };
    let mut first = String::new();
    for i in 0..9 {
        let result = service
            .query(&invoker, &json!({"segmentId":"1","output":"artifact"}))
            .await
            .unwrap();
        let path = result["artifact"]["path"].as_str().unwrap();
        assert!(std::path::Path::new(path).exists());
        if i == 0 {
            first = path.into();
        }
    }
    assert!(!std::path::Path::new(&first).exists());
    assert_eq!(service.artifacts.lock().unwrap().len(), 8);
}
fn workflow_server(base: &str) -> mcp_factory_core::McpProxyServer {
    let config = mcp_factory_core::ProxyConfig {
        base_url: base.into(),
        profiles: Some(vec!["workflow".into()]),
        ..Default::default()
    };
    super::super::test_support::server(config)
}
#[tokio::test]
async fn public_sales_and_finance_query_stream_gzip_tsv_via_internal_catalog() {
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let api = MockServer::start().await;
    let body = gzip(b"Country\tUnits\tProceeds\nUS\t2\t1.25\nGB\t9\t100\nUS\t3\t2.5\n");
    Mock::given(method("GET"))
        .and(path("/v1/salesReports"))
        .and(query_param("filter[vendorNumber]", "123"))
        .and(query_param("filter[reportDate]", "2026-10-10"))
        .and(query_param("filter[frequency]", "DAILY"))
        .and(query_param("filter[reportType]", "SALES"))
        .and(query_param("filter[reportSubType]", "SUMMARY"))
        .and(query_param("filter[version]", "1_0"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/a-gzip")
                .set_body_bytes(body.clone()),
        )
        .expect(1)
        .mount(&api)
        .await;
    Mock::given(method("GET"))
        .and(path("/v1/financeReports"))
        .and(query_param("filter[vendorNumber]", "123"))
        .and(query_param("filter[reportDate]", "2026-09"))
        .and(query_param("filter[regionCode]", "US"))
        .and(query_param("filter[reportType]", "FINANCIAL"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("Content-Type", "application/a-gzip")
                .set_body_bytes(body),
        )
        .expect(1)
        .mount(&api)
        .await;
    let server = workflow_server(&api.uri());
    assert!(server.tool_names().contains(&"report_query".into()));
    assert!(!server
        .tool_names()
        .contains(&"salesReports_getCollection".into()));
    let sales=server.invoke_tool("report_query",json!({"report":"sales","vendorNumber":"123","reportDate":"2026-10-10","reportVersion":"1_0","filters":{"Country":"US"},"metrics":["Units","Proceeds"]})).await.unwrap();
    let sales: Value = serde_json::from_str(&sales).unwrap();
    assert_eq!(sales["rowCount"], 2);
    assert_eq!(sales["sums"]["Units"], 5.0);
    assert_eq!(sales["sums"]["Proceeds"], 3.75);
    let finance=server.invoke_tool("report_query",json!({"report":"finance","vendorNumber":"123","reportDate":"2026-09","regionCode":"US","columns":["Country","Proceeds"],"filters":{"Country":"US"},"metrics":["Proceeds"],"output":"rows","limit":1})).await.unwrap();
    let finance: Value = serde_json::from_str(&finance).unwrap();
    assert_eq!(finance["rows"], json!([{"Country":"US","Proceeds":"1.25"}]));
    assert_eq!(finance["sums"]["Proceeds"], 3.75);
    assert_eq!(finance["droppedRows"], 1);
}
#[tokio::test]
async fn public_report_validation_precedes_http_and_preserves_apple_error_body() {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    let api = MockServer::start().await;
    let server = workflow_server(&api.uri());
    let result=server.invoke_tool("report_query",json!({"report":"sales","vendorNumber":"123","reportDate":"2026-10-10","frequency":"HOURLY"})).await.unwrap_err().to_string();
    assert!(result.to_lowercase().contains("validation"));
    let result = server
        .invoke_tool(
            "report_query",
            json!({"report":"finance","vendorNumber":"123","reportDate":"2026-09"}),
        )
        .await
        .unwrap_err()
        .to_string();
    assert!(result.contains("regionCode"));
    let result=server.invoke_tool("report_query",json!({"report":"sales","vendorNumber":"123","reportDate":"2026-10-10","dateRange":{"from":"2026-10-01","to":"2026-10-11"}})).await.unwrap_err().to_string();
    assert!(result.contains("actual report period"));
    assert!(api.received_requests().await.unwrap().is_empty());
    Mock::given(method("GET"))
        .and(path("/v1/salesReports"))
        .respond_with(ResponseTemplate::new(403).set_body_json(
            json!({"errors":[{"code":"FORBIDDEN_ERROR","detail":"Vendor access denied"}]}),
        ))
        .expect(1)
        .mount(&api)
        .await;
    let result = server
        .invoke_tool(
            "report_query",
            json!({"report":"sales","vendorNumber":"123","reportDate":"2026-10-10"}),
        )
        .await
        .unwrap_err()
        .to_string();
    let result: Value = serde_json::from_str(&result).unwrap();
    assert_eq!(result["details"]["status"], 403);
    assert!(result.to_string().contains("Vendor access denied"));
}
#[test]
fn retained_row_budget_is_strict_and_skips_oversized_candidates() {
    let exact = format!("A\n{}\n", "x".repeat(9_989));
    let result = state(json!({"output":"rows"}), exact.as_bytes())
        .unwrap()
        .result();
    assert_eq!(
        super::super::output_budget::json_bytes(&result["rows"]),
        9_999
    );
    assert_eq!(result["droppedRows"], 0);
    let oversized = format!("A\n{}\nsmall\n", "x".repeat(9_990));
    let result = state(json!({"output":"rows"}), oversized.as_bytes())
        .unwrap()
        .result();
    assert_eq!(result["rows"], json!([{"A":"small"}]));
    assert_eq!(result["droppedRows"], 1);
    assert_eq!(result["truncated"], true);
}
