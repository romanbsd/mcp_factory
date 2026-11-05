use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use mcp_factory_core::{
    async_trait,
    auth::{AuthProvider, HttpIdentity},
    CustomToolHandler, CustomToolSpec, McpProxyServer, OperationInvoker, ProxyError, ToolHints,
    ToolResult,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tower::ServiceExt;
use wiremock::{
    matchers::{header, method, path},
    Mock, MockServer, ResponseTemplate,
};
mod common;

struct Users;
struct User(&'static str);
#[async_trait]
impl AuthProvider for Users {
    fn requires_http_identity(&self) -> bool {
        true
    }
    async fn authenticate_http(&self, token: &str) -> Result<HttpIdentity, ProxyError> {
        let user = match token {
            "mcp-a" => "upstream-a",
            "mcp-b" => "upstream-b",
            _ => return Err(ProxyError::Validation("invalid".into())),
        };
        Ok(HttpIdentity {
            principal: user.into(),
            auth: Arc::new(User(user)),
        })
    }
    async fn apply_request_auth(
        &self,
        _: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ProxyError> {
        panic!("host identity must never be used")
    }
    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}
#[async_trait]
impl AuthProvider for User {
    async fn apply_request_auth(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::RequestBuilder, ProxyError> {
        Ok(request.bearer_auth(self.0))
    }
    fn api_key_query(&self) -> Option<(String, String)> {
        None
    }
}
struct Workflow;
#[async_trait]
impl CustomToolHandler for Workflow {
    async fn call(
        &self,
        invoker: &dyn OperationInvoker,
        _: Value,
    ) -> Result<ToolResult, ProxyError> {
        let raw = invoker.invoke_read_only("get_pet", json!({"petId":1}));
        let stream = invoker.stream_read_only("get_pet", json!({"petId":1}));
        let (raw, stream) = tokio::join!(raw, stream);
        assert!(stream?.status().is_success());
        raw
    }
}
fn request(token: Option<&str>, session: Option<&str>, body: Value) -> Request<Body> {
    let mut req = Request::builder()
        .method("POST")
        .uri("/mcp")
        .header("host", "localhost")
        .header("content-type", "application/json")
        .header("accept", "application/json, text/event-stream");
    if let Some(token) = token {
        req = req.header("authorization", format!("Bearer {token}"));
    }
    if let Some(session) = session {
        req = req
            .header("mcp-session-id", session)
            .header("mcp-protocol-version", "2025-03-26");
    }
    req.body(Body::from(body.to_string())).unwrap()
}
fn initialize() -> Value {
    json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"test","version":"1"}}})
}

fn workflow_tool() -> CustomToolSpec {
    CustomToolSpec {
        name: "workflow".into(),
        description: "workflow".into(),
        input_schema: json!({"type":"object"}),
        hints: ToolHints {
            read_only: Some(true),
            ..Default::default()
        },
        handler: Arc::new(Workflow),
    }
}

#[tokio::test]
async fn http_users_have_isolated_sessions_and_concurrent_workflow_credentials() {
    let upstream = MockServer::start().await;
    for user in ["upstream-a", "upstream-b"] {
        Mock::given(method("GET"))
            .and(path("/pets/1"))
            .and(header("authorization", format!("Bearer {user}").as_str()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"user":user})))
            .expect(2)
            .mount(&upstream)
            .await;
    }
    let mut tool = common::rest_get_pet_tool();
    tool.hints.read_only = Some(true);
    let server = McpProxyServer::builder(common::proxy_config(&upstream.uri()))
        .auth_provider(Some(Arc::new(Users)))
        .tools(&[tool])
        .unwrap()
        .custom_tools(&[workflow_tool()])
        .unwrap()
        .build()
        .unwrap();
    assert!(server.invoke_tool("workflow", json!({})).await.is_err());
    let cancel = CancellationToken::new();
    let app = mcp_factory_core::transport::http_router(server, cancel.clone());
    for token in [None, Some("invalid")] {
        let response = app
            .clone()
            .oneshot(request(token, None, initialize()))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
    let mut sessions = Vec::new();
    for token in ["mcp-a", "mcp-b"] {
        let response = app
            .clone()
            .oneshot(request(Some(token), None, initialize()))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let session = response.headers()["mcp-session-id"]
            .to_str()
            .unwrap()
            .to_string();
        // Drain the initialization response before sending initialized.
        to_bytes(response.into_body(), 65536).await.unwrap();
        let response = app
            .clone()
            .oneshot(request(
                Some(token),
                Some(&session),
                json!({"jsonrpc":"2.0","method":"notifications/initialized"}),
            ))
            .await
            .unwrap();
        assert!(response.status().is_success());
        sessions.push(session);
    }
    let response = app
        .clone()
        .oneshot(request(
            Some("mcp-b"),
            Some(&sessions[0]),
            json!({"jsonrpc":"2.0","id":3,"method":"tools/list"}),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let call = json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"workflow","arguments":{}}});
    let (a, b) = tokio::join!(
        app.clone()
            .oneshot(request(Some("mcp-a"), Some(&sessions[0]), call.clone())),
        app.clone()
            .oneshot(request(Some("mcp-b"), Some(&sessions[1]), call))
    );
    for (response, user) in [(a, "upstream-a"), (b, "upstream-b")] {
        let response = response.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = to_bytes(response.into_body(), 65536).await.unwrap();
        let text = String::from_utf8(bytes.to_vec()).unwrap();
        assert!(text.contains(user), "{text}");
    }
    cancel.cancel();
}

#[tokio::test]
async fn stateless_http_calls_are_independent_and_keep_caller_credentials() {
    let upstream = MockServer::start().await;
    for user in ["upstream-a", "upstream-b"] {
        Mock::given(method("GET"))
            .and(path("/pets/1"))
            .and(header("authorization", format!("Bearer {user}").as_str()))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"user":user})))
            .expect(3)
            .mount(&upstream)
            .await;
    }
    let mut config = common::proxy_config(&upstream.uri());
    config.http_stateless = true;
    let mut tool = common::rest_get_pet_tool();
    tool.hints.read_only = Some(true);
    let server = McpProxyServer::builder(config)
        .auth_provider(Some(Arc::new(Users)))
        .tools(&[tool])
        .unwrap()
        .custom_tools(&[workflow_tool()])
        .unwrap()
        .build()
        .unwrap();
    let cancellation = CancellationToken::new();
    let app = mcp_factory_core::transport::http_router(server, cancellation.clone());
    let response = app
        .clone()
        .oneshot(request(Some("mcp-a"), None, initialize()))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get("mcp-session-id").is_none());
    to_bytes(response.into_body(), 65536).await.unwrap();
    // No initialization, cookie or shared session is needed for subsequent calls.
    let call = json!({"jsonrpc":"2.0","id":9,"method":"tools/call","params":{"name":"get_pet","arguments":{"petId":1}}});
    for call in [
        call.clone(),
        json!({"jsonrpc":"2.0","id":10,"method":"tools/call","params":{"name":"workflow","arguments":{}}}),
    ] {
        let mut a = request(Some("mcp-a"), None, call.clone());
        let mut b = request(Some("mcp-b"), None, call.clone());
        for req in [&mut a, &mut b] {
            req.headers_mut()
                .insert("mcp-protocol-version", "2025-03-26".parse().unwrap());
        }
        let (a, b) = tokio::join!(app.clone().oneshot(a), app.clone().oneshot(b));
        for (response, user) in [(a, "upstream-a"), (b, "upstream-b")] {
            let response = response.unwrap();
            assert_eq!(response.status(), StatusCode::OK);
            assert!(response.headers().get("mcp-session-id").is_none());
            let text = String::from_utf8(
                to_bytes(response.into_body(), 65536)
                    .await
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
            assert!(text.contains(user), "{text}");
        }
    }
    assert_eq!(
        app.clone()
            .oneshot(request(None, None, call))
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    for method in ["GET", "DELETE"] {
        let req = Request::builder()
            .method(method)
            .uri("/mcp")
            .header("host", "localhost")
            .header("authorization", "Bearer mcp-a")
            .header("accept", "text/event-stream")
            .body(Body::empty())
            .unwrap();
        assert_eq!(
            app.clone().oneshot(req).await.unwrap().status(),
            StatusCode::METHOD_NOT_ALLOWED
        );
    }
    cancellation.cancel();
}
