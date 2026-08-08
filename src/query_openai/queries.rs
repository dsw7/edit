use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::json;

use crate::utils::load_api_key;

use super::response::deserialize_json_response;
use super::structs::OpenAIResults;

fn get_structured_output_schema() -> serde_json::Value {
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

fn get_system_prompt() -> &'static str {
    "You are a helpful programming assistant.

IMPORTANT: Do not wrap your response in backticks (```). Output the code
directly without markdown code fences.

Output:
- description_of_what_was_done: brief summary of what you did
- code: your updated code
"
}

pub fn write_new_code(model: String, prompt: String) -> anyhow::Result<OpenAIResults> {
    let api_key = load_api_key("OPENAI_API_KEY")?;

    let request_body = json!({
        "input": prompt,
        "instructions": get_system_prompt(),
        "model": model,
        "store": false,
        "text": get_structured_output_schema(),
    });

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request_body)
        .send()?;

    let raw_json = response.text()?;
    deserialize_json_response(raw_json)
}
