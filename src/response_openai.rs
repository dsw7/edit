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
enum RawResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub completion: String,
}

pub struct OpenAIError {
    pub errmsg: String,
}

pub enum OpenAIResponse {
    OpenAIResults(OpenAIResults),
    OpenAIError(OpenAIError),
}

pub fn unpack_response(response_text: String) -> Result<OpenAIResponse, Box<dyn error::Error>> {
    let response: RawResponse = serde_json::from_str(&response_text)?;

    let results = match response {
        RawResponse::Success(success) => OpenAIResponse::OpenAIResults(OpenAIResults {
            input_tokens: success.usage.input_tokens,
            output_tokens: success.usage.output_tokens,
            completion: success.output[0].content[0].text.clone(),
        }),
        RawResponse::Error(error) => OpenAIResponse::OpenAIError(OpenAIError {
            errmsg: error.error.message,
        }),
    };

    Ok(results)
}
