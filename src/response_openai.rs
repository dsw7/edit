use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Content {
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct Output {
    pub status: String,
    pub content: Vec<Content>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub input_tokens: u32,
    pub output_tokens: u32,
}

#[derive(Deserialize, Debug)]
pub struct SuccessResponse {
    pub usage: Usage,
    pub output: Vec<Output>,
}

#[derive(Deserialize, Debug)]
pub struct NestedError {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: NestedError,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum OpenAIResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}
