#[path = "high_level/mod.rs"]
mod high_level;

use mcp_factory_core::CustomToolSpec;

pub fn build_custom_tools() -> Vec<CustomToolSpec> {
    high_level::build_tools()
}
