//! Shared server registration for workflow HTTP integration tests.

use mcp_factory_core::{McpProxyServer, ProxyConfig};

pub(super) fn server(config: ProxyConfig) -> McpProxyServer {
    McpProxyServer::builder(config.clone())
        .operation_catalog(crate::tools::operation_catalog())
        .unwrap()
        .tool_profiles(&crate::tools::build_tool_profiles())
        .unwrap()
        .custom_tools(&super::build_custom_tools(&config))
        .unwrap()
        .resources(&super::build_resources())
        .unwrap()
        .build()
        .unwrap()
}
