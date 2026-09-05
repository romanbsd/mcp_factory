mod extensions;
mod resources;
mod tools;

use mcp_factory_core::{run_oauth_login, McpProxyServer, ProxyConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = ProxyConfig::load_runtime(ProxyConfig {
        base_url: "https://androidpublisher.googleapis.com".to_string(),
        server_name: "google-play-mcp".to_string(),
        server_version: env!("CARGO_PKG_VERSION").to_string(),
        transport: "stdio".parse().unwrap_or_default(),
        ..ProxyConfig::default()
    })?;
    // A config.toml may omit identity fields. Restore the generation-time
    // defaults so a partial config doesn't silently drop them.
    if config.server_version == ProxyConfig::default().server_version {
        config.server_version = env!("CARGO_PKG_VERSION").to_string();
    }
    if config.base_url.is_empty() {
        config.base_url = "https://androidpublisher.googleapis.com".to_string();
    }
    if config.server_name.is_empty() {
        config.server_name = "google-play-mcp".to_string();
    }
    if config.transport == ProxyConfig::default().transport {
        config.transport = "stdio".parse().unwrap_or_default();
    }

    if std::env::args().any(|a| a == "--auth-login") {
        return run_oauth_login(&config).await.map_err(Into::into);
    }

    let tools = tools::build_tools();
    let custom_tools = extensions::build_custom_tools();
    let resources = resources::build_resources();
    let server = McpProxyServer::builder(config)
        .tools(&tools)?
        .custom_tools(&custom_tools)?
        .resources(&resources)?
        .build()?;

    server.run().await?;
    Ok(())
}
