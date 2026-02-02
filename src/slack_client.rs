use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;

const SLACK_API_BASE: &str = "https://slack.com/api";

pub struct SlackClient {
    client: Client,
    token: String,
}

impl SlackClient {
    pub fn from_env() -> Result<Self> {
        let token = std::env::var("SLACK_BOT_TOKEN")
            .map_err(|_| anyhow!("SLACK_BOT_TOKEN environment variable is not set"))?;
        Ok(Self {
            client: Client::new(),
            token,
        })
    }

    pub async fn post(&self, method: &str, body: Value) -> Result<Value> {
        let url = format!("{}/{}", SLACK_API_BASE, method);
        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let data: Value = resp.json().await?;

        if !status.is_success() {
            return Err(anyhow!("Slack API HTTP error {}: {}", status, data));
        }

        if data.get("ok") != Some(&Value::Bool(true)) {
            let error = data
                .get("error")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown");
            return Err(anyhow!("Slack API error in {}: {} ({})", method, error, data));
        }

        Ok(data)
    }
}
