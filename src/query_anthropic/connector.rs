use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;
use serde_json::Value;

pub struct AnthropicConnector<'a> {
    api_key: String,
    base_url: &'a str,
    client: Client,
}

impl AnthropicConnector<'_> {
    pub fn try_new(api_key: String) -> anyhow::Result<Self> {
        let connection_timeout = Duration::from_secs(60);

        let client = Client::builder().timeout(connection_timeout).build()?;
        let base_url = "https://api.anthropic.com";

        Ok(AnthropicConnector {
            client,
            base_url,
            api_key,
        })
    }

    pub fn query_messages_api(&self, request_body: Value) -> anyhow::Result<String> {
        let response = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("Content-Type", "application/json")
            .header("anthropic-version", "2023-06-01")
            .header("X-Api-Key", &self.api_key)
            .json(&request_body)
            .send()?;

        let raw_json = response
            .text()
            .context("failed to decode response body to string")?;

        Ok(raw_json)
    }
}
