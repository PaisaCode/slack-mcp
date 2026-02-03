mod slack_client;
mod tools;
mod types;

use anyhow::Result;
use rmcp::ServiceExt;
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

    let transport = std::env::var("TRANSPORT").unwrap_or_else(|_| "stdio".to_string());

    match transport.as_str() {
        "stdio" => {
            let service = server.serve(rmcp::transport::stdio()).await?;
            tracing::info!("slack-mcp server running (stdio)");
            service.waiting().await?;
        }
        "sse" => {
            let host = std::env::var("SSE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = std::env::var("SSE_PORT").unwrap_or_else(|_| "8080".to_string());
            let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse()?;

            tracing::info!("slack-mcp SSE server listening on {}", addr);

            let ct = rmcp::transport::sse_server::SseServer::serve(addr)
                .await?
                .with_service(move || server.clone());

            tokio::signal::ctrl_c().await?;
            tracing::info!("Shutting down SSE server");
            ct.cancel();
        }
        other => {
            anyhow::bail!("Unknown TRANSPORT: '{}'. Use 'stdio' or 'sse'.", other);
        }
    }

    Ok(())
}
