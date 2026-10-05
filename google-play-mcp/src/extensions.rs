#[path = "high_level/mod.rs"]
mod high_level;

use mcp_factory_core::{CustomToolSpec, ProxyConfig};

pub fn build_custom_tools(config: &ProxyConfig) -> Vec<CustomToolSpec> {
    let mut tools = high_level::build_tools();
    tools.extend(high_level::installs::build_tools(config));
    tools
}
