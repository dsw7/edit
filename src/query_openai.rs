use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{self, json};

use crate::params::Parameters;
use crate::utils::load_api_key;

// set up request ---------------------------------------------------------------------------------

fn get_request_body(params: &Parameters) -> serde_json::Value {
    let structured_output_schema: serde_json::Value = json!({
        "format": {
            "type": "json_schema",
            "name": "updated_code",
            "strict": true,
            "schema": {
                "type": "object",
                "properties": {
                    "description_of_changes": { "type": "string" },
                    "code": { "type": "string" }
                },
                "required": ["description_of_changes", "code"],
                "additionalProperties": false
            }
        }
    });

    let system_prompt: &str = "You are a helpful assistant.";

    json!({
        "input": params.prompt,
        "model": params.model,
        "instructions": system_prompt,
        "text": structured_output_schema,
    })
}

// unpack response --------------------------------------------------------------------------------

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
struct StructuredOutput {
    code: String,
    description_of_changes: String,
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

// ------------------------------------------------------------------------------------------------

pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub completion: String,
}

pub fn query_openai(params: &Parameters) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    let api_key = load_api_key("OPENAI_API_KEY")?;
    let client = Client::builder()
        .timeout(Duration::from_secs(params.client_timeout))
        .build()?;

    let request_body = get_request_body(&params);
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request_body)
        .send()?;

    let response_text = response.text()?;
    println!("{response_text}");
    let response: RawResponse = serde_json::from_str(&response_text)?;

    match response {
        RawResponse::Success(success) => {
            let results = OpenAIResults {
                input_tokens: success.usage.input_tokens,
                output_tokens: success.usage.output_tokens,
                completion: extract_output_text(&success)?,
            };
            Ok(results)
        }
        RawResponse::Error(error) => Err(error.error.message.into()),
    }
}
