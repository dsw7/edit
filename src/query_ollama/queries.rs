use anyhow::Context;

use super::connector::OllamaConnector;
use super::requests::request_body_validate_prompt;
use super::response::deserialize_prompt_validation_response;
use super::structs::ValidationResults;

use crate::configurations::Configs;

pub fn is_valid_prompt(params: &Configs, prompt: &str) -> anyhow::Result<ValidationResults> {
    let connector = OllamaConnector::try_new(&params.ollama_host, params.ollama_port)?;
    connector.try_handshake()?;

    let request_body = request_body_validate_prompt(params, prompt);
    let raw_json = connector
        .query_generate_api(request_body)
        .context("failed to query Ollama")?;

    deserialize_prompt_validation_response(raw_json)
}
