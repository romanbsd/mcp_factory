use std::sync::Arc;

use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolRequestParams, CallToolResult, ContentBlock, ListResourcesResult, ListToolsResult,
    PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult, Resource,
    ResourceContents, ServerCapabilities, ServerInfo, Tool,
};
use rmcp::service::{NotificationContext, RequestContext, RoleServer};
use rmcp::ErrorData as McpError;
use serde_json::Value;

use crate::config::ProxyConfig;
use crate::error::ProxyError;
use crate::graphql::GraphQLProxyExecutor;
use crate::resources::{ResourceRegistry, ResourceSpec};
use crate::rest::RestProxyExecutor;
use crate::tools::{ExecutionKind, ToolRegistry, ToolSpec};

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
        let rest = RestProxyExecutor::new(self.config.clone())?;
        let graphql = GraphQLProxyExecutor::new(self.config.clone())?;
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

        let result = match &tool.execution {
            ExecutionKind::Rest(_) => self.inner.rest.execute(tool, args).await,
            ExecutionKind::GraphQL(_) => self.inner.graphql.execute(tool, args).await,
        };

        match result {
            Ok(text) => Ok(CallToolResult::success(vec![ContentBlock::text(text)])),
            Err(err) => Ok(CallToolResult::error(vec![ContentBlock::text(err.to_string())])),
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
                Resource::new(&r.uri, &r.name).with_description(&r.description).with_mime_type(&r.mime_type)
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
        Ok(ReadResourceResult::new(vec![
            ResourceContents::text(resource.content, &request.uri)
                .with_mime_type(&resource.mime_type),
        ]))
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

fn tool_spec_to_rmcp(spec: &ToolSpec) -> Result<Tool, ProxyError> {
    let schema_obj = spec
        .input_schema
        .as_object()
        .cloned()
        .ok_or_else(|| ProxyError::Validation("tool input_schema must be an object".to_string()))?;
    Ok(Tool::new(
        spec.name.clone(),
        spec.description.clone(),
        Arc::new(schema_obj),
    ))
}
