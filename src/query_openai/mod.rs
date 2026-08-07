mod request;
mod response;
mod structs;

use std::time::Duration;

use reqwest::blocking::Client;

use crate::utils::load_api_key;

use request::set_up_request_body;
use response::deserialize_json_response;

pub use structs::{OpenAIParams, OpenAIResults};

pub fn run_query(params: &OpenAIParams) -> anyhow::Result<OpenAIResults> {
    let api_key = load_api_key("OPENAI_API_KEY")?;
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let request_body = set_up_request_body(params);
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request_body)
        .send()?;

    let raw_json = response.text()?;
    deserialize_json_response(raw_json)
}

#[cfg(test)]
mod tests {
    use super::{OpenAIParams, run_query};

    fn assert_error_message(params: &OpenAIParams, expected_error: &str) {
        let result = run_query(&params);
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), expected_error);
    }

    #[test]
    fn test_invalid_model() {
        let params = OpenAIParams::from_str("foobar", "What is 3 + 5?");
        assert_error_message(&params, "The requested model 'foobar' does not exist.");
    }

    #[test]
    fn test_incompatible_model() {
        let params = OpenAIParams::from_str("gpt-3.5-turbo", "What is 3 + 5?");
        assert_error_message(
            &params,
            "Invalid parameter: 'text.format' of type 'json_schema' is not supported with model version `gpt-3.5-turbo`.",
        );
    }

    #[test]
    fn test_valid_query() {
        let params = OpenAIParams::from_str("gpt-4o", "Print 'hello world' in Python.");
        let result = run_query(&params).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }
}
