use schemars::JsonSchema;
use serde::Deserialize;

// ─── Messaging ──────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PostMessageParams {
    #[schemars(description = "Slack channel ID (e.g. C0123456789). Falls back to SLACK_DEFAULT_CHANNEL env var if omitted.")]
    pub channel: Option<String>,

    #[schemars(description = "Message text. Supports Slack mrkdwn formatting.")]
    pub text: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ReplyToMessageParams {
    #[schemars(description = "Slack channel ID where the parent message lives.")]
    pub channel: String,

    #[schemars(description = "Timestamp (ts) of the parent message to reply to.")]
    pub thread_ts: String,

    #[schemars(description = "Reply message text. Supports Slack mrkdwn formatting.")]
    pub text: String,

    #[schemars(description = "If true, also post the reply to the channel (not just the thread). Defaults to false.")]
    pub reply_broadcast: Option<bool>,
}

// ─── Canvas ─────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateCanvasParams {
    #[schemars(description = "Title for the new canvas.")]
    pub title: String,

    #[schemars(description = "Markdown content for the canvas body.")]
    pub markdown: String,

    #[schemars(description = "Optional channel ID. If provided, creates a channel-bound canvas; otherwise creates a standalone canvas.")]
    pub channel_id: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateCanvasParams {
    #[schemars(description = "The canvas ID to update.")]
    pub canvas_id: String,

    #[schemars(description = "New markdown content to replace the entire canvas body.")]
    pub markdown: String,
}

// ─── Channels ───────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListChannelsParams {
    #[schemars(description = "Maximum number of channels to return. Default 100, max 1000.")]
    pub limit: Option<u32>,

    #[schemars(description = "Pagination cursor returned from a previous call.")]
    pub cursor: Option<String>,

    #[schemars(description = "Comma-separated channel types: public_channel, private_channel, mpim, im. Default: public_channel.")]
    pub types: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListMessagesParams {
    #[schemars(description = "Channel ID to fetch message history from.")]
    pub channel: String,

    #[schemars(description = "Maximum number of messages to return. Default 20, max 1000.")]
    pub limit: Option<u32>,

    #[schemars(description = "Pagination cursor from a previous response.")]
    pub cursor: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetThreadRepliesParams {
    #[schemars(description = "Channel ID containing the thread.")]
    pub channel: String,

    #[schemars(description = "Timestamp (ts) of the parent message.")]
    pub ts: String,

    #[schemars(description = "Maximum number of replies to return. Default 100, max 1000.")]
    pub limit: Option<u32>,

    #[schemars(description = "Pagination cursor from a previous response.")]
    pub cursor: Option<String>,
}

// ─── Reactions ──────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AddReactionParams {
    #[schemars(description = "Channel ID where the message to react to was posted.")]
    pub channel: String,

    #[schemars(description = "Timestamp of the message to add a reaction to.")]
    pub timestamp: String,

    #[schemars(description = "Emoji name without colons (e.g. 'thumbsup', not ':thumbsup:').")]
    pub name: String,
}

// ─── Users ──────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUsersParams {
    #[schemars(description = "Maximum number of users to return. Default 100, max 1000.")]
    pub limit: Option<u32>,

    #[schemars(description = "Pagination cursor from a previous response.")]
    pub cursor: Option<String>,
}
