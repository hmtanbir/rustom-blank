use reqwest::Client;
use serde_json::json;

pub struct SlackNotification;

impl SlackNotification {
    /// Generic send helper
    async fn send(webhook_url: &str, text: &str) -> Result<(), reqwest::Error> {
        let client = Client::new();
        let payload = json!({ "text": text });
        let response = client.post(webhook_url).json(&payload).send().await?;
        if !response.status().is_success() {
            tracing::warn!("Failed to send Slack notification: {:?}", response.status());
        }
        Ok(())
    }

    /// Notify using an environment variable name for the webhook URL
    async fn notify_with_env(env_var: &str, message: &str) -> Result<(), reqwest::Error> {
        let webhook_url = std::env::var(env_var).unwrap_or_default();
        if webhook_url.is_empty() {
            return Ok(());
        }
        Self::send(&webhook_url, message).await
    }

    /// Notify on unexpected errors (uses SLACK_ERROR_WEBHOOK_URL)
    pub async fn notify_error(message: &str) -> Result<(), reqwest::Error> {
        Self::notify_with_env("SLACK_ERROR_WEBHOOK_URL", message).await
    }

    /// Notify on successful registration (uses SLACK_WEBHOOK_URL)
    pub async fn notify_registration(message: &str) -> Result<(), reqwest::Error> {
        Self::notify_with_env("SLACK_WEBHOOK_URL", message).await
    }

    /// Send a custom message to an explicit webhook URL (useful for dynamic channels)
    pub async fn notify_custom(webhook_url: &str, message: &str) -> Result<(), reqwest::Error> {
        if webhook_url.is_empty() {
            return Ok(());
        }
        Self::send(webhook_url, message).await
    }
}
