use std::time::Duration;

use anyhow::Context;
use reqwest::blocking::Client;
use serde_json::json;

use crate::utils::load_api_key;

use super::response::deserialize_json_response;
use super::structs::OpenAIResults;

fn schema_structured_output_code_generation() -> serde_json::Value {
    json!({
        "format": {
            "type": "json_schema",
            "name": "updated_code",
            "strict": true,
            "schema": {
                "type": "object",
                "properties": {
                    "description_of_what_was_done": { "type": "string" },
                    "code": { "type": "string" }
                },
                "required": ["description_of_what_was_done", "code"],
                "additionalProperties": false
            }
        }
    })
}

fn system_prompt_code_generation() -> &'static str {
    "You are a helpful programming assistant.

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- description_of_what_was_done: brief summary of what you did
- code: your updated code
"
}

fn query_responses_api(request_body: serde_json::Value) -> anyhow::Result<String> {
    let api_key = load_api_key("OPENAI_API_KEY")?;

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request_body)
        .send()?;

    let raw_json = response
        .text()
        .context("Failed to decode response body to string")?;

    Ok(raw_json)
}

pub fn write_new_code(model: String, prompt: &str) -> anyhow::Result<OpenAIResults> {
    let request_body = json!({
        "input": prompt,
        "instructions": system_prompt_code_generation(),
        "model": model,
        "store": false,
        "text": schema_structured_output_code_generation(),
    });

    let raw_json = query_responses_api(request_body).context("Failed to write code with OpenAI")?;
    deserialize_json_response(raw_json)
}

fn user_prompt_code_edit(prompt: &str, code_block: &str) -> String {
    format!(
        "Take the instructions:
```plaintext
{prompt}
```
And apply them to the code:
```
{code_block}
```"
    )
}

pub fn edit_code_block(
    model: &str,
    prompt: &str,
    code_block: &str,
) -> anyhow::Result<OpenAIResults> {
    let request_body = json!({
        "input": user_prompt_code_edit(prompt, code_block),
        "instructions": system_prompt_code_generation(),
        "model": model,
        "store": false,
        "text": schema_structured_output_code_generation(),
    });

    let raw_json = query_responses_api(request_body).context("Failed to edit code with OpenAI")?;
    deserialize_json_response(raw_json)
}

#[cfg(test)]
mod tests {
    use super::{edit_code_block, write_new_code};

    #[test]
    fn test_write_new_code_invalid_model() {
        let model = "foobar";
        let prompt = "What is 3 + 5?";

        let result = write_new_code(model.to_string(), prompt);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "The requested model 'foobar' does not exist."
        );
    }

    #[test]
    fn test_write_new_code_incompatible_model() {
        let model = "gpt-3.5-turbo";
        let prompt = "What is 3 + 5?";

        let result = write_new_code(model.to_string(), prompt);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Invalid parameter: 'text.format' of type 'json_schema' is not supported with model version `gpt-3.5-turbo`."
        );
    }

    #[test]
    fn test_write_new_code_valid_query() {
        let model = "gpt-4o";
        let prompt = "Print 'hello world' in Python.";
        let result = write_new_code(model.to_string(), prompt).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }

    #[test]
    fn test_edit_code_block_valid_query() {
        let model = "gpt-4o";
        let prompt = "Fix the code such that it prints 'hello world'";
        let code_block = "print('hello world'";
        let result = edit_code_block(model, prompt, code_block).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }
}
