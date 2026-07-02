use rmcp::transport::TokioChildProcess;
use rmcp::ServiceExt;

#[tokio::test]
async fn stdio_subprocess_handshake_and_lists_tools() -> anyhow::Result<()> {
    let bin = env!("CARGO_BIN_EXE_mcp-factory-stdio-smoke");
    let transport = TokioChildProcess::new(tokio::process::Command::new(bin))?;
    let client = ().serve(transport).await?;
    let tools = client.list_all_tools().await?;
    assert!(tools.iter().any(|tool| tool.name == "get_pet"));
    client.cancel().await?;
    Ok(())
}

#[tokio::test]
async fn stdio_subprocess_reads_resource() -> anyhow::Result<()> {
    let bin = env!("CARGO_BIN_EXE_mcp-factory-stdio-smoke");
    let transport = TokioChildProcess::new(tokio::process::Command::new(bin))?;
    let client = ().serve(transport).await?;
    let resources = client.list_all_resources().await?;
    // smoke server has no resources; ensure protocol round-trip succeeds
    assert!(resources.is_empty());
    client.cancel().await?;
    Ok(())
}
