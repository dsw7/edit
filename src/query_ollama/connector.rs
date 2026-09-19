use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;
use serde_json::Value;

pub struct OllamaConnector {
    base_url: String,
    client: Client,
}

impl OllamaConnector {
    pub fn try_new(host: &str, port: u16) -> anyhow::Result<Self> {
        let connection_timeout = Duration::from_secs(60);

        let client = Client::builder().timeout(connection_timeout).build()?;
        let base_url = format!("http://{host}:{port}");

        Ok(OllamaConnector { base_url, client })
    }

    pub fn try_handshake(&self) -> anyhow::Result<()> {
        self.client
            .get(&self.base_url)
            .send()
            .context("handshake with Ollama failed")?;

        Ok(())
    }

    pub fn query_generate_api(&self, request_body: Value) -> anyhow::Result<String> {
        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?;

        let raw_json = response
            .text()
            .context("failed to decode response body to string")?;

        Ok(raw_json)
    }
}
