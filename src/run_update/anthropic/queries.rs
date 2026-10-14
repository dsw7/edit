use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;

use super::requests::request_edit_code_block;
use super::response::{AnthropicResults, deserialize_json_response};
use crate::run_update::configs::Configs;

pub fn query_messages_api(configs: &Configs, language: &str) -> anyhow::Result<AnthropicResults> {
    let connection_timeout = Duration::from_secs(60);
    let client = Client::builder().timeout(connection_timeout).build()?;

    let request_body = request_edit_code_block(configs, language);

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("Content-Type", "application/json")
        .header("anthropic-version", "2023-06-01")
        .header("X-Api-Key", &configs.api_key)
        .json(&request_body)
        .send()?;

    let raw_json = response
        .text()
        .context("failed to decode response body to string")?;

    deserialize_json_response(&raw_json)
}
