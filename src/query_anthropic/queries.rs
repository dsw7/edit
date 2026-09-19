use anyhow::Context;

use crate::utils::load_api_key;

use super::connector::AnthropicConnector;
use super::requests::{request_edit_code_block, request_write_new_code};
use super::response::{AnthropicResults, deserialize_json_response};

pub fn write_new_code(
    max_tokens: u16,
    model: &str,
    prompt: &str,
) -> anyhow::Result<AnthropicResults> {
    let api_key = load_api_key("ANTHROPIC_API_KEY")?;
    let connector = AnthropicConnector::try_new(api_key)?;

    let request_body = request_write_new_code(max_tokens, model, prompt);
    let raw_json = connector
        .query_messages_api(request_body)
        .context("failed to write code")?;

    deserialize_json_response(raw_json)
}

pub fn edit_code_block(
    code_block: &str,
    max_tokens: u16,
    model: &str,
    prompt: &str,
) -> anyhow::Result<AnthropicResults> {
    let api_key = load_api_key("ANTHROPIC_API_KEY")?;
    let connector = AnthropicConnector::try_new(api_key)?;

    let request_body = request_edit_code_block(code_block, max_tokens, model, prompt);
    let raw_json = connector
        .query_messages_api(request_body)
        .context("failed to edit code")?;

    deserialize_json_response(raw_json)
}

#[cfg(test)]
mod tests {
    use super::{edit_code_block, write_new_code};

    #[test]
    fn test_write_new_code_invalid_model() {
        let model = "foobar";
        let prompt = "What is 3 + 5?";

        let result = write_new_code(4096, model, prompt);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "model: foobar");
    }

    #[test]
    fn test_write_new_code_valid_query() {
        let model = "claude-haiku-4-5";
        let prompt = "Print 'hello world' in Python.";
        let result = write_new_code(4096, model, prompt).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }

    #[test]
    fn test_edit_code_block_valid_query() {
        let model = "claude-haiku-4-5";
        let prompt = "Fix the code such that it prints 'hello world'";
        let code_block = "print('hello world'";
        let result = edit_code_block(code_block, 4096, model, prompt).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }
}
