mod resources;
mod tools;

use mcp_factory_core::{McpProxyServer, ProxyConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = if std::path::Path::new("config.toml").exists() {
        ProxyConfig::load("config.toml")?
    } else {
        ProxyConfig::default()
    };
    config = config.merge_env()?;
    if config.base_url.is_empty() {
        config.base_url = "http://127.0.0.1:8080/graphql".to_string();
    }
    if config.server_name.is_empty() {
        config.server_name = "graphql-mcp".to_string();
    }

    let tools = tools::build_tools();
    let resources = resources::build_resources();
    let server = McpProxyServer::builder(config)
        .tools(&tools)?
        .resources(&resources)?
        .build()?;

    server.run().await?;
    Ok(())
}