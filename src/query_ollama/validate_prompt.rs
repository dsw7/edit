use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;

use super::requests::request_body_validate_prompt;
use super::response::deserialize_prompt_validation_response;
use super::structs::ValidationResults;

use crate::configurations::Configs;

struct OllamaConnector {
    base_url: String,
    client: Client,
}

impl OllamaConnector {
    fn try_new(host: &str, port: u16) -> anyhow::Result<Self> {
        let connection_timeout = Duration::from_secs(60);

        let client = Client::builder().timeout(connection_timeout).build()?;
        let base_url = format!("http://{host}:{port}");

        Ok(OllamaConnector { base_url, client })
    }

    fn try_handshake(&self) -> anyhow::Result<()> {
        self.client
            .get(&self.base_url)
            .send()
            .context("handshake with Ollama failed")?;

        Ok(())
    }

    fn query_generate_api(&self, request_body: serde_json::Value) -> anyhow::Result<String> {
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

pub fn is_valid_prompt(params: &Configs, prompt: &str) -> anyhow::Result<ValidationResults> {
    let connector = OllamaConnector::try_new(&params.ollama_host, params.ollama_port)?;
    connector.try_handshake()?;

    let request_body = request_body_validate_prompt(params, prompt);
    let raw_json = connector
        .query_generate_api(request_body)
        .context("failed to query Ollama")?;

    deserialize_prompt_validation_response(raw_json)
}
