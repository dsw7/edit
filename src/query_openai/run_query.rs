use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::json;

use crate::utils::load_api_key;

use super::response::{OpenAIResults, deserialize_json_response};

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

fn set_up_request_body(prompt: &String, model: &String) -> serde_json::Value {
    let structured_output_schema = get_structured_output_schema();
    let system_prompt = get_system_prompt();

    json!({
        "input": prompt,
        "model": model,
        "instructions": system_prompt,
        "text": structured_output_schema,
    })
}

pub fn run_query(prompt: &String, model: &String) -> anyhow::Result<OpenAIResults> {
    let api_key = load_api_key("OPENAI_API_KEY")?;
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let request_body = set_up_request_body(prompt, model);
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request_body)
        .send()?;

    let raw_json = response.text()?;
    deserialize_json_response(raw_json)
}
