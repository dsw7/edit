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

pub fn write_new_code(model: String, prompt: String) -> anyhow::Result<OpenAIResults> {
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

fn user_prompt_code_edit(prompt: String, code_block: &str) -> String {
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
    model: String,
    prompt: String,
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
