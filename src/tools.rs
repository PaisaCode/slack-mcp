use rmcp::{
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*, tool, tool_handler, tool_router, ServerHandler,
};
use serde_json::json;
use std::borrow::Cow;
use std::future::Future;
use std::sync::Arc;

use crate::slack_client::SlackClient;
use crate::types::*;

#[derive(Clone)]
pub struct SlackTools {
    client: Arc<SlackClient>,
    default_channel: Option<String>,
    tool_router: ToolRouter<SlackTools>,
}

impl SlackTools {
    pub fn new(client: SlackClient, default_channel: Option<String>) -> Self {
        Self {
            client: Arc::new(client),
            default_channel,
            tool_router: Self::tool_router(),
        }
    }

    fn resolve_channel(&self, channel: Option<String>) -> Result<String, ErrorData> {
        channel
            .filter(|c| !c.is_empty())
            .or_else(|| self.default_channel.clone())
            .ok_or_else(|| ErrorData {
                code: ErrorCode::INVALID_PARAMS,
                message: Cow::from(
                    "channel is required (or set SLACK_DEFAULT_CHANNEL env var)",
                ),
                data: None,
            })
    }

    fn err(e: anyhow::Error) -> ErrorData {
        ErrorData {
            code: ErrorCode(-32603),
            message: Cow::from(e.to_string()),
            data: None,
        }
    }
}

#[tool_router]
impl SlackTools {
    #[tool(description = "Post a message to a Slack channel")]
    async fn post_message(
        &self,
        Parameters(params): Parameters<PostMessageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let channel = self.resolve_channel(params.channel)?;
        let body = json!({
            "channel": channel,
            "text": params.text,
        });
        let data = self.client.post("chat.postMessage", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Reply to a message in a thread")]
    async fn reply_to_message(
        &self,
        Parameters(params): Parameters<ReplyToMessageParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut body = json!({
            "channel": params.channel,
            "text": params.text,
            "thread_ts": params.thread_ts,
        });
        if let Some(true) = params.reply_broadcast {
            body["reply_broadcast"] = json!(true);
        }
        let data = self.client.post("chat.postMessage", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Create a Slack canvas with markdown content. If channel_id is provided, creates a channel-bound canvas; otherwise a standalone canvas.")]
    async fn create_canvas(
        &self,
        Parameters(params): Parameters<CreateCanvasParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let document_content = json!({
            "type": "markdown",
            "markdown": params.markdown,
        });

        let data = if let Some(channel_id) = params.channel_id.filter(|c| !c.is_empty()) {
            self.client
                .post(
                    "conversations.canvases.create",
                    json!({
                        "channel_id": channel_id,
                        "document_content": document_content,
                    }),
                )
                .await
        } else {
            self.client
                .post(
                    "canvases.create",
                    json!({
                        "title": params.title,
                        "document_content": document_content,
                    }),
                )
                .await
        }
        .map_err(Self::err)?;

        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Update an existing Slack canvas by replacing all content with new markdown")]
    async fn update_canvas(
        &self,
        Parameters(params): Parameters<UpdateCanvasParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = json!({
            "canvas_id": params.canvas_id,
            "changes": [{
                "operation": "replace",
                "document_content": {
                    "type": "markdown",
                    "markdown": params.markdown,
                },
            }],
        });
        let data = self.client.post("canvases.edit", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "List Slack channels the bot has access to")]
    async fn list_channels(
        &self,
        Parameters(params): Parameters<ListChannelsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut body = json!({});
        if let Some(limit) = params.limit {
            body["limit"] = json!(limit);
        }
        if let Some(ref cursor) = params.cursor {
            body["cursor"] = json!(cursor);
        }
        if let Some(ref types) = params.types {
            body["types"] = json!(types);
        }
        let data = self.client.post("conversations.list", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Get recent messages from a Slack channel")]
    async fn list_messages(
        &self,
        Parameters(params): Parameters<ListMessagesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut body = json!({ "channel": params.channel });
        if let Some(limit) = params.limit {
            body["limit"] = json!(limit);
        }
        if let Some(ref cursor) = params.cursor {
            body["cursor"] = json!(cursor);
        }
        let data = self
            .client
            .post("conversations.history", body)
            .await
            .map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Get all replies in a message thread")]
    async fn get_thread_replies(
        &self,
        Parameters(params): Parameters<GetThreadRepliesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut body = json!({
            "channel": params.channel,
            "ts": params.ts,
        });
        if let Some(limit) = params.limit {
            body["limit"] = json!(limit);
        }
        if let Some(ref cursor) = params.cursor {
            body["cursor"] = json!(cursor);
        }
        let data = self
            .client
            .post("conversations.replies", body)
            .await
            .map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "Add an emoji reaction to a message")]
    async fn add_reaction(
        &self,
        Parameters(params): Parameters<AddReactionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let body = json!({
            "channel": params.channel,
            "timestamp": params.timestamp,
            "name": params.name,
        });
        let data = self.client.post("reactions.add", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }

    #[tool(description = "List users in the Slack workspace")]
    async fn get_users(
        &self,
        Parameters(params): Parameters<GetUsersParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut body = json!({});
        if let Some(limit) = params.limit {
            body["limit"] = json!(limit);
        }
        if let Some(ref cursor) = params.cursor {
            body["cursor"] = json!(cursor);
        }
        let data = self.client.post("users.list", body).await.map_err(Self::err)?;
        Ok(CallToolResult::success(vec![Content::text(data.to_string())]))
    }
}

#[tool_handler]
impl ServerHandler for SlackTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Slack integration tools. Requires SLACK_BOT_TOKEN env var. \
                 Optionally set SLACK_DEFAULT_CHANNEL for a default channel."
                    .to_string(),
            ),
        }
    }
}
