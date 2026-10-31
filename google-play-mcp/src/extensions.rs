/// Server-owned provider selection; generated startup detects this optional hook.
pub fn build_auth_provider(
    config: &mcp_factory_core::ProxyConfig,
) -> Result<
    Option<std::sync::Arc<dyn mcp_factory_core::auth::AuthProvider>>,
    mcp_factory_core::ProxyError,
> {
    mcp_factory_provider_auth::from_config(config)
}

#[path = "high_level/mod.rs"]
mod high_level;

use mcp_factory_core::{CustomToolSpec, ProxyConfig};

pub fn build_custom_tools(config: &ProxyConfig) -> Vec<CustomToolSpec> {
    let mut tools = high_level::build_tools();
    tools.extend(high_level::installs::build_tools(config));
    tools
}
