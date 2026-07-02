use std::env::{self, VarError};
use std::error::Error;
use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::{self, json};

use crate::params::Parameters;

fn get_api_key() -> Result<String, String> {
    const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

    match env::var(OPENAI_API_KEY) {
        Ok(key) => Ok(key),
        Err(VarError::NotPresent) => Err("OpenAI API key not found".to_string()),
        Err(err) => Err(format!("Failed to retrieve API key: {:?}", err)),
    }
}

fn get_request_body(params: &Parameters) -> serde_json::Value {
    let input = json!([
        { "role": "system", "content": "You are a helpful assistant." },
        { "role": "user", "content": params.prompt }
    ]);

    json!({ "model": params.model, "input": input })
}

pub fn query_openai(params: &Parameters) -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;
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
    println!("{}", response_text);

    Ok(())
}
