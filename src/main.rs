mod slack_client;
mod tools;
mod types;

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::EnvFilter;

use crate::slack_client::SlackClient;
use crate::tools::SlackTools;

#[tokio::main]
async fn main() -> Result<()> {
    // Tracing MUST go to stderr — stdout is the MCP stdio transport channel.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting slack-mcp server");

    let client = SlackClient::from_env()?;
    let default_channel = std::env::var("SLACK_DEFAULT_CHANNEL")
        .ok()
        .filter(|s| !s.is_empty());

    let server = SlackTools::new(client, default_channel);
    let service = server.serve(stdio()).await?;

    tracing::info!("slack-mcp server running");
    service.waiting().await?;

    Ok(())
}
