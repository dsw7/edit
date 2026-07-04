use std::time::Duration;

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::{self, json};

use crate::params::Parameters;
use crate::utils::load_api_key;

// set up request ---------------------------------------------------------------------------------

fn get_structured_output() -> serde_json::Value {
    json!({
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
    })
}

fn get_request_body(params: &Parameters) -> serde_json::Value {
    let input = json!([
        { "role": "system", "content": "You are a helpful assistant." },
        { "role": "user", "content": params.prompt }
    ]);

    json!({ "model": params.model, "input": input , "text": get_structured_output() })
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
    content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct Content {
    text: String,
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

fn extract_output_text(response: &SuccessResponse) -> String {
    for object in &response.output {
        if object.status == "completed" {
            let content = &object.content[0];
            return content.text.clone();
        }
    }

    String::from("No results")
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
    let response: RawResponse = serde_json::from_str(&response_text)?;

    match response {
        RawResponse::Success(success) => {
            let results = OpenAIResults {
                input_tokens: success.usage.input_tokens,
                output_tokens: success.usage.output_tokens,
                completion: extract_output_text(&success),
            };
            Ok(results)
        }
        RawResponse::Error(error) => Err(error.error.message.into()),
    }
}
