use std::sync::Arc;

use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, ListResourcesResult, ListToolsResult,
    Meta, PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult, Resource,
    ResourceContents, ServerCapabilities, ServerInfo, Tool, ToolAnnotations,
};
use rmcp::service::{NotificationContext, RequestContext, RoleServer};
use rmcp::ErrorData as McpError;
use serde_json::Value;

use crate::config::ProxyConfig;
use crate::error::ProxyError;
use crate::graphql::GraphQLProxyExecutor;
use crate::resources::{ResourceRegistry, ResourceSpec};
use crate::rest::RestProxyExecutor;
use crate::tools::{ExecutionKind, ToolBody, ToolRegistry, ToolResult, ToolSpec};

#[derive(Clone)]
pub struct McpProxyServer {
    inner: Arc<McpProxyServerInner>,
}

struct McpProxyServerInner {
    config: ProxyConfig,
    tools: ToolRegistry,
    resources: ResourceRegistry,
    rest: RestProxyExecutor,
    graphql: GraphQLProxyExecutor,
}

pub struct McpProxyServerBuilder {
    config: ProxyConfig,
    tools: ToolRegistry,
    resources: ResourceRegistry,
}

impl McpProxyServer {
    pub fn builder(config: ProxyConfig) -> McpProxyServerBuilder {
        McpProxyServerBuilder::new(config)
    }

    pub fn config(&self) -> &ProxyConfig {
        &self.inner.config
    }

    pub async fn run(self) -> Result<(), ProxyError> {
        crate::transport::run(self).await
    }

    pub fn tool_count(&self) -> usize {
        self.inner.tools.len()
    }

    pub fn tool_names(&self) -> Vec<String> {
        self.inner.tools.iter().map(|t| t.name.clone()).collect()
    }

    pub async fn invoke_tool(&self, name: &str, args: Value) -> Result<String, ProxyError> {
        let tool = self
            .inner
            .tools
            .get(name)
            .ok_or_else(|| ProxyError::ToolNotFound(name.to_string()))?;
        self.inner.tools.validate(name, &args)?;
        let result = self.dispatch(tool, args).await?;
        if result.is_error {
            return Err(ProxyError::Other(result.into_text()));
        }
        Ok(result.into_text())
    }

    /// Route a validated tool call to its executor. Single source of dispatch
    /// for both `invoke_tool` and the MCP `call_tool` handler.
    async fn dispatch(&self, tool: &ToolSpec, args: Value) -> Result<ToolResult, ProxyError> {
        match &tool.execution {
            ExecutionKind::Rest(_) => self.inner.rest.execute(tool, args).await,
            ExecutionKind::GraphQL(_) => self.inner.graphql.execute(tool, args).await,
        }
    }

    pub fn read_resource_content(&self, uri: &str) -> Result<String, ProxyError> {
        let resource = self
            .inner
            .resources
            .get(uri)
            .ok_or_else(|| ProxyError::ResourceNotFound(uri.to_string()))?;
        Ok(resource.content.to_string())
    }
}

impl McpProxyServerBuilder {
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            config,
            tools: ToolRegistry::new(),
            resources: ResourceRegistry::new(),
        }
    }

    pub fn tools(mut self, tools: &[ToolSpec]) -> Result<Self, ProxyError> {
        self.tools.register_many(tools.iter().cloned())?;
        Ok(self)
    }

    pub fn resources(mut self, resources: &[ResourceSpec]) -> Result<Self, ProxyError> {
        self.resources.register_many(resources.iter().cloned())?;
        Ok(self)
    }

    pub fn build(self) -> Result<McpProxyServer, ProxyError> {
        // One shared client so the connection pool / TLS setup is reused across
        // auth, REST, and GraphQL calls (reqwest::Client is Arc-backed).
        let http = reqwest::Client::builder()
            .timeout(self.config.timeout())
            .build()?;
        // Only launch the browser login flow on stdio (local) servers; an HTTP
        // server may be remote/headless, where popping a browser is wrong.
        let interactive = !matches!(self.config.transport, crate::config::TransportMode::Http);
        let auth =
            crate::auth::auth_provider_from_config(&self.config.auth, http.clone(), interactive)?;
        let rest = RestProxyExecutor::new(
            http.clone(),
            self.config.base_url.clone(),
            Arc::clone(&auth),
        );
        let graphql =
            GraphQLProxyExecutor::new(http, self.config.base_url.clone(), Arc::clone(&auth));
        Ok(McpProxyServer {
            inner: Arc::new(McpProxyServerInner {
                config: self.config,
                tools: self.tools,
                resources: self.resources,
                rest,
                graphql,
            }),
        })
    }
}

impl ServerHandler for McpProxyServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .build(),
        );
        if !self.inner.config.server_name.is_empty() {
            info.server_info.name = self.inner.config.server_name.clone();
        }
        if !self.inner.config.server_version.is_empty() {
            info.server_info.version = self.inner.config.server_version.clone();
        }
        info
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let tools = self
            .inner
            .tools
            .iter()
            .map(tool_spec_to_rmcp)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ListToolsResult {
            tools,
            ..Default::default()
        })
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, McpError> {
        let tool = self
            .inner
            .tools
            .get(&request.name)
            .ok_or_else(|| ProxyError::ToolNotFound(request.name.to_string()))?;

        let args = request
            .arguments
            .map(Value::Object)
            .unwrap_or_else(|| Value::Object(Default::default()));

        if let Err(err) = self.inner.tools.validate(&request.name, &args) {
            return Ok(CallToolResult::error(vec![ContentBlock::text(
                err.to_string(),
            )]));
        }

        match self.dispatch(tool, args).await {
            Ok(result) => Ok(result_to_call(result, &request.name)),
            Err(err) => Ok(CallToolResult::error(vec![ContentBlock::text(
                err.to_string(),
            )])),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        let resources = self
            .inner
            .resources
            .iter()
            .map(|r| {
                Resource::new(&r.uri, &r.name)
                    .with_description(&r.description)
                    .with_mime_type(&r.mime_type)
            })
            .collect();
        Ok(ListResourcesResult {
            resources,
            ..Default::default()
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        let resource = self
            .inner
            .resources
            .get(&request.uri)
            .ok_or_else(|| ProxyError::ResourceNotFound(request.uri.clone()))?;
        Ok(ReadResourceResult::new(vec![ResourceContents::text(
            resource.content,
            &request.uri,
        )
        .with_mime_type(&resource.mime_type)]))
    }

    async fn on_initialized(&self, _context: NotificationContext<RoleServer>) {
        tracing::info!(
            server = %self.inner.config.server_name,
            tools = self.inner.tools.len(),
            resources = self.inner.resources.iter().count(),
            "MCP proxy server initialized"
        );
    }
}

/// Text bodies larger than this are handed back as an inline embedded resource
/// rather than a bare text block, so clients can treat the payload as a document
/// instead of chat text. The `embedded://` URI is intentionally not readable via
/// `read_resource`; the bytes are already carried in this content block.
const LARGE_TEXT_BYTES: usize = 256 * 1024;

/// Assemble the full MCP `CallToolResult` from an executor's `ToolResult`:
/// the right content block, `structuredContent`, `isError`, and header `_meta`.
fn result_to_call(result: ToolResult, tool_name: &str) -> CallToolResult {
    let block = body_to_content(result.body, tool_name);
    let mut call = if result.is_error {
        CallToolResult::error(vec![block])
    } else {
        CallToolResult::success(vec![block])
    };
    call.structured_content = result.structured;
    if !result.meta.is_empty() {
        call.meta = Some(Meta(result.meta));
    }
    call
}

/// Wrap a tool body in the most appropriate MCP content block: small text as
/// text, large text as an embedded text resource, binary as image/audio or an
/// embedded blob resource (base64) with its MIME type.
fn body_to_content(body: ToolBody, tool_name: &str) -> ContentBlock {
    match body {
        ToolBody::Text(text) if text.len() > LARGE_TEXT_BYTES => ContentBlock::resource(
            ResourceContents::text(text, format!("embedded://tool/{tool_name}/response")),
        ),
        ToolBody::Text(text) => ContentBlock::text(text),
        ToolBody::Binary { data, mime } => {
            use base64::Engine;
            let encoded = base64::engine::general_purpose::STANDARD.encode(&data);
            if mime.starts_with("image/") {
                ContentBlock::image(encoded, mime)
            } else if mime.starts_with("audio/") {
                ContentBlock::audio(encoded, mime)
            } else {
                ContentBlock::resource(ResourceContents::blob(
                    encoded,
                    format!("tool://{tool_name}/response"),
                ))
            }
        }
    }
}

fn tool_spec_to_rmcp(spec: &ToolSpec) -> Result<Tool, ProxyError> {
    let schema_obj =
        spec.input_schema.as_object().cloned().ok_or_else(|| {
            ProxyError::Validation("tool input_schema must be an object".to_string())
        })?;
    let mut tool = Tool::new(
        spec.name.clone(),
        spec.description.clone(),
        Arc::new(schema_obj),
    );

    let hints = &spec.hints;
    tool.title = hints.title.clone();
    if let Some(schema) = &hints.output_schema {
        let obj = schema.as_object().cloned().ok_or_else(|| {
            ProxyError::Validation("tool output_schema must be an object".to_string())
        })?;
        tool.output_schema = Some(Arc::new(obj));
    }
    // Emit annotations only when at least one hint is set, so we don't override
    // client defaults with a wall of `null`s.
    if hints.title.is_some()
        || hints.read_only.is_some()
        || hints.destructive.is_some()
        || hints.idempotent.is_some()
        || hints.open_world.is_some()
    {
        tool.annotations = Some(ToolAnnotations::from_raw(
            hints.title.clone(),
            hints.read_only,
            hints.destructive,
            hints.idempotent,
            hints.open_world,
        ));
    }
    Ok(tool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::RestOperation;
    use crate::tools::ToolHints;
    use serde_json::json;

    fn rest_spec(hints: ToolHints) -> ToolSpec {
        ToolSpec {
            name: "get_pet".to_string(),
            description: "Get a pet".to_string(),
            input_schema: json!({"type": "object", "properties": {}}),
            execution: ExecutionKind::Rest(RestOperation {
                method: "GET".to_string(),
                path_template: "/pets".to_string(),
                params: vec![],
                body_fields: vec![],
                content_type: None,
                raw_body: false,
            }),
            hints,
        }
    }

    #[test]
    fn maps_hints_to_tool_metadata() {
        let spec = rest_spec(ToolHints {
            title: Some("Get pet".to_string()),
            output_schema: Some(json!({"type": "object"})),
            read_only: Some(true),
            idempotent: Some(true),
            open_world: Some(true),
            ..Default::default()
        });
        let tool = tool_spec_to_rmcp(&spec).unwrap();
        assert_eq!(tool.title.as_deref(), Some("Get pet"));
        assert!(tool.output_schema.is_some());
        let ann = tool.annotations.unwrap();
        assert_eq!(ann.read_only_hint, Some(true));
        assert_eq!(ann.idempotent_hint, Some(true));
        assert_eq!(ann.open_world_hint, Some(true));
    }

    #[test]
    fn no_annotations_when_no_hints() {
        let tool = tool_spec_to_rmcp(&rest_spec(ToolHints::default())).unwrap();
        assert!(tool.annotations.is_none());
        assert!(tool.output_schema.is_none());
    }

    #[test]
    fn result_to_call_carries_structured_error_and_meta() {
        let mut meta = serde_json::Map::new();
        meta.insert("http.location".to_string(), json!("/pets/1"));
        let result = ToolResult {
            body: ToolBody::Text("boom".to_string()),
            structured: Some(json!({"status": 500})),
            meta,
            is_error: true,
        };
        let call = result_to_call(result, "get_pet");
        assert_eq!(call.is_error, Some(true));
        assert_eq!(call.structured_content.unwrap()["status"], 500);
        assert!(call.meta.is_some());
    }

    #[test]
    fn large_text_becomes_resource_block() {
        let big = "x".repeat(LARGE_TEXT_BYTES + 1);
        let block = body_to_content(ToolBody::Text(big), "get_pet");
        let ContentBlock::Resource(resource) = block else {
            panic!("expected resource block");
        };
        let ResourceContents::TextResourceContents { uri, .. } = resource.resource else {
            panic!("expected text resource");
        };
        assert_eq!(uri, "embedded://tool/get_pet/response");
    }
}
