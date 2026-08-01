use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{self, json};

use crate::utils::load_api_key;

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

#[derive(Deserialize, Debug)]
struct SuccessResponse {
    usage: Usage,
    output: Vec<Output>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct Output {
    status: String,
    content: Vec<TextOrRefusal>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TextOrRefusal {
    TextResponse { text: String },
    RefusalResponse { refusal: String },
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: Error,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

fn extract_output_text(response: &SuccessResponse) -> Result<String, String> {
    for object in &response.output {
        if object.status == "completed" {
            return match &object.content[0] {
                TextOrRefusal::TextResponse { text } => Ok(text.clone()),
                TextOrRefusal::RefusalResponse { refusal } => Err(refusal.clone()),
            };
        }
    }

    Err("Query never completed".to_string())
}

pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}

fn deserialize_raw_json(raw_json: String) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    let response: RawResponse = serde_json::from_str(&raw_json)?;

    match response {
        RawResponse::Success(success) => {
            let text = extract_output_text(&success)?;
            let structured_output: StructuredOutput = serde_json::from_str(&text)?;

            let results = OpenAIResults {
                input_tokens: success.usage.input_tokens,
                output_tokens: success.usage.output_tokens,
                code: structured_output.code,
                description_of_what_was_done: structured_output.description_of_what_was_done,
            };
            Ok(results)
        }
        RawResponse::Error(error) => Err(error.error.message.into()),
    }
}

pub fn query_openai(
    prompt: &String,
    model: &String,
) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
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
    deserialize_raw_json(raw_json)
}
