use std::error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Content {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Output {
    status: String,
    content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct SuccessResponse {
    usage: Usage,
    output: Vec<Output>,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: Error,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum OpenAIResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

pub fn unpack_response(response_text: String) -> Result<(), Box<dyn error::Error>> {
    let results: OpenAIResponse = serde_json::from_str(&response_text)?;

    match results {
        OpenAIResponse::Success(success) => {
            println!(
                "Input tokens: {:?} {:?}",
                success.usage.input_tokens, success.usage.output_tokens
            );
        }
        OpenAIResponse::Error(error) => {
            println!("Error: {:?}", error.error.message);
        }
    }

    Ok(())
}
