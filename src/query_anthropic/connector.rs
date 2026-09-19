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
}
