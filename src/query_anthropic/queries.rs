use anyhow::Context;

use crate::configurations::Configs;

use super::connector::AnthropicConnector;
use super::requests::request_edit_code_block;
use super::response::{AnthropicResults, deserialize_json_response};

pub fn edit_code_block(configs: &Configs, language: &str) -> anyhow::Result<AnthropicResults> {
    let connector = AnthropicConnector::try_new(&configs.api_key)?;

    let request_body = request_edit_code_block(configs, language);
    let raw_json = connector
        .query_messages_api(request_body)
        .context("failed to edit code")?;

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
