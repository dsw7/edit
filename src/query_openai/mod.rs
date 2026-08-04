mod request;
mod response;

use std::time::Duration;

use reqwest::blocking::Client;

use crate::utils::load_api_key;

use request::set_up_request_body;
pub use response::OpenAIResults;
use response::deserialize_json_response;

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
