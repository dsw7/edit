use std::error::Error;
use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::{self, json};

use crate::params::Parameters;
use crate::response_openai::unpack_response;
use crate::utils::load_api_key;

fn get_request_body(params: &Parameters) -> serde_json::Value {
    let input = json!([
        { "role": "system", "content": "You are a helpful assistant." },
        { "role": "user", "content": params.prompt }
    ]);

    json!({ "model": params.model, "input": input })
}

pub fn query_openai(params: &Parameters) -> Result<(), Box<dyn Error>> {
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
    let _ = unpack_response(response_text);

    Ok(())
}
