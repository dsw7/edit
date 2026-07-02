use std::env::{self, VarError};
use std::error::Error;
use std::time::Duration;

use reqwest::blocking::Client;
use serde_json::json;

fn get_api_key() -> Result<String, String> {
    const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

    match env::var(OPENAI_API_KEY) {
        Ok(key) => Ok(key),
        Err(VarError::NotPresent) => Err("OpenAI API key not found".to_string()),
        Err(err) => Err(format!("Failed to retrieve API key: {:?}", err)),
    }
}

pub fn query_openai() -> Result<(), Box<dyn Error>> {
    let api_key = get_api_key()?;

    let input = json!([
        { "role": "system", "content": "You are a helpful assistant." },
        { "role": "user", "content": "What is 3 + 5?" }
    ]);

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({ "model": "gpt-4o", "input": input }))
        .send()?;

    let response_text = response.text()?;
    println!("{}", response_text);
    Ok(())
}
