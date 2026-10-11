mod common;

use mcp_factory_core::{
    ExecutionKind, McpProxyServer, ParamBinding, ParamLocation, RestOperation, ToolSpec,
};
use serde_json::{json, Value};
use wiremock::matchers::{body_json, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn apps_tool(method: &str) -> ToolSpec {
    ToolSpec {
        name: "apps".to_string(),
        description: "Exercise OpenAPI query array encoding".to_string(),
        input_schema: json!({
            "type": "object",
            "properties": {
                "fields[apps]": {"type": "array", "items": {"type": "string"},
                    "x-mcp-query-explode": false},
                "tags": {"type": "array", "items": {"type": "string"}},
                "data": {"type": "object"}
            },
            "required": ["fields[apps]", "tags"],
            "additionalProperties": false
        }),
        execution: ExecutionKind::Rest(RestOperation {
            method: method.to_string(),
            path_template: "/v1/apps".to_string(),
            params: vec![
                ParamBinding {
                    name: "fields[apps]".to_string(),
                    location: ParamLocation::Query,
                },
                ParamBinding {
                    name: "tags".to_string(),
                    location: ParamLocation::Query,
                },
            ],
            body_fields: if method == "POST" {
                vec![
                    "fields[apps]".to_string(),
                    "tags".to_string(),
                    "data".to_string(),
                ]
            } else {
                vec![]
            },
            content_type: Some("application/json".to_string()),
            raw_body: false,
            media: None,
        }),
        hints: Default::default(),
    }
}

fn server(api: &MockServer, method: &str) -> McpProxyServer {
    McpProxyServer::builder(common::proxy_config(&api.uri()))
        .tools_owned(vec![apps_tool(method)])
        .unwrap()
        .build()
        .unwrap()
}

#[tokio::test]
async fn nonexploded_openapi_fields_and_default_repeated_tags_encode_reserved_characters() {
    let api = MockServer::start().await;
    let expected = json!({"data": [{"type": "apps", "id": "app-1", "attributes": {
        "name": "Example", "bundleId": "com.example.app"
    }}]});
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(expected.clone()))
        .expect(1)
        .mount(&api)
        .await;
    let result = server(&api, "GET")
        .invoke_tool(
            "apps",
            json!({
                "fields[apps]": ["name", "bundleId"],
                "tags": ["one&two=three?four#five/six", "two words+plus"]
            }),
        )
        .await
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&result).unwrap(), expected);
    let requests = api.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    let pairs: Vec<_> = request
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(
        pairs,
        vec![
            ("fields[apps]".to_string(), "name,bundleId".to_string()),
            (
                "tags".to_string(),
                "one&two=three?four#five/six".to_string()
            ),
            ("tags".to_string(), "two words+plus".to_string()),
        ]
    );
    assert_eq!(
        pairs
            .iter()
            .filter(|(key, _)| key == "fields[apps]")
            .count(),
        1
    );
    assert_eq!(pairs.iter().filter(|(key, _)| key == "tags").count(), 2);
    assert_eq!(request.url.query().unwrap(),
        "fields%5Bapps%5D=name%2CbundleId&tags=one%26two%3Dthree%3Ffour%23five%2Fsix&tags=two+words%2Bplus");
    assert!(request.body.is_empty());
}

#[tokio::test]
async fn query_conversion_keeps_json_body_arrays_and_reserved_values_intact() {
    let api = MockServer::start().await;
    let input = json!({
        "fields[apps]": ["name&alias", "bundleId/value"],
        "tags": ["first,tag", "second=tag"],
        "data": {"attributes": {"labels": ["body&value", "body,value"]}}
    });
    let expected = json!({"accepted": true, "id": "app-2"});
    Mock::given(method("POST"))
        .and(path("/v1/apps"))
        .and(body_json(input.clone()))
        .respond_with(ResponseTemplate::new(201).set_body_json(expected.clone()))
        .expect(1)
        .mount(&api)
        .await;
    let result = server(&api, "POST")
        .invoke_tool("apps", input.clone())
        .await
        .unwrap();
    assert_eq!(serde_json::from_str::<Value>(&result).unwrap(), expected);
    let requests = api.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    assert_eq!(
        serde_json::from_slice::<Value>(&requests[0].body).unwrap(),
        input
    );
    let pairs: Vec<_> = requests[0]
        .url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    assert_eq!(
        pairs,
        vec![
            (
                "fields[apps]".to_string(),
                "name&alias,bundleId/value".to_string()
            ),
            ("tags".to_string(), "first,tag".to_string()),
            ("tags".to_string(), "second=tag".to_string()),
        ]
    );
    assert_eq!(
        requests[0].url.query().unwrap(),
        "fields%5Bapps%5D=name%26alias%2CbundleId%2Fvalue&tags=first%2Ctag&tags=second%3Dtag"
    );
}

#[tokio::test]
async fn nonexploded_annotation_does_not_relax_the_array_input_schema() {
    let api = MockServer::start().await;
    let proxy = server(&api, "GET");
    for input in [
        json!({"fields[apps]": "name,bundleId", "tags": ["one", "two"]}),
        json!({"fields[apps]": ["name", "bundleId"], "tags": "one,two"}),
        json!({"fields[apps]": ["name", 42], "tags": ["one", "two"]}),
    ] {
        let error = proxy
            .invoke_tool("apps", input)
            .await
            .unwrap_err()
            .to_string();
        assert!(
            error.starts_with("validation error:") && error.contains("not of type"),
            "{error}"
        );
    }
    assert!(api.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn shared_transfer_budget_blocks_gzip_but_leaves_json_control_requests_available() {
    use std::io::Write;
    use std::time::Duration;

    use flate2::{write::GzEncoder, Compression};
    use mcp_factory_core::shared_transfer_budget;
    use wiremock::matchers::query_param;

    let api = MockServer::start().await;
    let report = "App,Downloads\nExample,42\n";
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(report.as_bytes()).unwrap();
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .and(query_param("tags", "binary"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(encoder.finish().unwrap(), "application/a-gzip"),
        )
        .expect(1)
        .mount(&api)
        .await;
    let control_response = json!({"data": [{"type": "apps", "id": "control-app"}]});
    Mock::given(method("GET"))
        .and(path("/v1/apps"))
        .and(query_param("tags", "control"))
        .respond_with(ResponseTemplate::new(200).set_body_json(control_response.clone()))
        .expect(1)
        .mount(&api)
        .await;
    let proxy = server(&api, "GET");
    let budget = shared_transfer_budget();
    assert_eq!(budget.available_permits(), 2);
    let permits = budget.acquire_many_owned(2).await.unwrap();
    let binary = proxy.invoke_tool(
        "apps",
        json!({"fields[apps]": ["name"], "tags": ["binary"]}),
    );
    tokio::pin!(binary);
    assert!(
        tokio::time::timeout(Duration::from_millis(100), &mut binary)
            .await
            .is_err()
    );
    // Receipt establishes that the pending call reached the response processing path.
    assert_eq!(api.received_requests().await.unwrap().len(), 1);
    let control = tokio::time::timeout(
        Duration::from_secs(2),
        proxy.invoke_tool(
            "apps",
            json!({"fields[apps]": ["name"], "tags": ["control"]}),
        ),
    )
    .await
    .expect("JSON control request must not wait for transfer permits")
    .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&control).unwrap(),
        control_response
    );
    assert_eq!(shared_transfer_budget().available_permits(), 0);
    assert!(tokio::time::timeout(Duration::from_millis(20), &mut binary)
        .await
        .is_err());
    drop(permits);
    let result = tokio::time::timeout(Duration::from_secs(2), &mut binary)
        .await
        .expect("gzip response should finish after permits are released")
        .unwrap();
    assert_eq!(result, report);
    assert_eq!(shared_transfer_budget().available_permits(), 2);
}
