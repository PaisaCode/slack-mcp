use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

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

// ─── Lists ──────────────────────────────────────────────

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateListParams {
    #[schemars(description = "Name of the new list.")]
    pub name: String,

    #[schemars(description = "Description of the list.")]
    pub description: String,

    #[schemars(description = "If true, the list behaves as a to-do list with done/not-done state per item.")]
    pub todo_mode: Option<bool>,

    #[schemars(description = "Column schema definition for the list. A JSON object whose keys are column IDs and values describe each column (type, label, options, etc.). Omit to create a simple list with default columns.")]
    pub schema: Option<Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateListParams {
    #[schemars(description = "The list ID to update.")]
    pub id: String,

    #[schemars(description = "New name for the list.")]
    pub name: Option<String>,

    #[schemars(description = "New description for the list.")]
    pub description: Option<String>,

    #[schemars(description = "Enable or disable to-do mode.")]
    pub todo_mode: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CreateListItemParams {
    #[schemars(description = "The list ID to add the item to.")]
    pub list_id: String,

    #[schemars(description = "JSON object of initial field values keyed by column ID. Values depend on column type (text, number, date, user, etc.).")]
    pub initial_fields: Option<Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListListItemsParams {
    #[schemars(description = "The list ID to fetch items from.")]
    pub list_id: String,

    #[schemars(description = "Maximum number of items to return.")]
    pub limit: Option<u32>,

    #[schemars(description = "Pagination cursor from a previous response.")]
    pub cursor: Option<String>,

    #[schemars(description = "If true, include archived items.")]
    pub archived: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetListItemParams {
    #[schemars(description = "The list ID containing the item.")]
    pub list_id: String,

    #[schemars(description = "The item ID to retrieve.")]
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateListItemParams {
    #[schemars(description = "The list ID containing the item.")]
    pub list_id: String,

    #[schemars(description = "JSON object of field values to update, keyed by column ID.")]
    pub cells: Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteListItemParams {
    #[schemars(description = "The list ID containing the item.")]
    pub list_id: String,

    #[schemars(description = "The item ID to delete.")]
    pub id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteListItemsParams {
    #[schemars(description = "The list ID containing the items.")]
    pub list_id: String,

    #[schemars(description = "Array of item IDs to delete.")]
    pub ids: Value,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetListAccessParams {
    #[schemars(description = "The list ID to modify access for.")]
    pub list_id: String,

    #[schemars(description = "Access level to grant: 'read', 'write', or 'owner'.")]
    pub access_level: String,

    #[schemars(description = "Channel IDs to grant access to.")]
    pub channel_ids: Option<Value>,

    #[schemars(description = "User IDs to grant access to.")]
    pub user_ids: Option<Value>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct DeleteListAccessParams {
    #[schemars(description = "The list ID to revoke access from.")]
    pub list_id: String,

    #[schemars(description = "Channel IDs to revoke access from.")]
    pub channel_ids: Option<Value>,

    #[schemars(description = "User IDs to revoke access from.")]
    pub user_ids: Option<Value>,
}
