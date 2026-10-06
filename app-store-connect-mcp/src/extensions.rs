use mcp_factory_core::{CustomToolSpec, ProxyConfig};

/// Handwritten tool extension point. This file is created once and preserved
/// by subsequent generation runs.
pub fn build_custom_tools(_config: &ProxyConfig) -> Vec<CustomToolSpec> {
    Vec::new()
}
