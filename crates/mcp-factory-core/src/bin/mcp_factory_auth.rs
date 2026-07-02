use std::path::PathBuf;

use clap::{Parser, Subcommand};
use mcp_factory_core::{oauth_logout, oauth_status, run_oauth_login, ProxyConfig};

#[derive(Parser)]
#[command(name = "mcp-factory-auth", about = "OAuth2 login for MCP Factory servers")]
struct Cli {
    #[arg(long, default_value = "config.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run interactive OAuth2 Authorization Code + PKCE login
    Login,
    /// Show stored token metadata (never prints raw tokens)
    Status,
    /// Delete stored tokens
    Logout,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = ProxyConfig::load(&cli.config)?.merge_env()?;
    match cli.command {
        Command::Login => run_oauth_login(&config).await?,
        Command::Status => oauth_status(&config).await?,
        Command::Logout => oauth_logout(&config).await?,
    }
    Ok(())
}
