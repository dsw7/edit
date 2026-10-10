use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;

use crate::configurations::Configs;

use super::requests::request_edit_code_block;
use super::response::{AnthropicResults, deserialize_json_response};

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

    deserialize_json_response(raw_json)
}

#[cfg(test)]
mod tests {
    use super::{AnthropicParams, edit_code_block};

    #[test]
    fn test_edit_code_block_valid_query() {
        let code_block = "print('hello world'";
        let prompt = "Fix the code such that it prints 'hello world'";
        let params = AnthropicParams::default();
        let result = edit_code_block(code_block, prompt, &params).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }
}
