use std::env;

use anyhow::Context;

use super::connector::AnthropicConnector;
use super::params::AnthropicParams;
use super::requests::request_edit_code_block;
use super::response::{AnthropicResults, deserialize_json_response};

fn load_anthropic_api_key() -> anyhow::Result<String> {
    let env_var_key = "ANTHROPIC_API_KEY";

    let env_var_value = env::var(env_var_key).context(format!(
        "failed to load environment variable: {env_var_key}"
    ))?;

    Ok(env_var_value)
}

pub fn edit_code_block(params: &AnthropicParams) -> anyhow::Result<AnthropicResults> {
    let api_key = load_anthropic_api_key()?;
    let connector = AnthropicConnector::try_new(api_key)?;

    let request_body = request_edit_code_block(params);
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
