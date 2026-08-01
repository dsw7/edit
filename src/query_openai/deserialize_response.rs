use anyhow::{Context, bail};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

// success

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
    content: Vec<serde_json::Value>,
}

// error

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: Error,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

// content

enum ContentType {
    Text(String),
    Refusal(String),
    Incomplete,
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

fn extract_completed_object(response: &SuccessResponse) -> ContentType {
    for object in &response.output {
        if object.status == "completed" {
            for content in &object.content {
                if content["type"] == "output_text" {
                    return ContentType::Text(content["text"].to_string());
                }

                if content["type"] == "refusal" {
                    return ContentType::Refusal(content["refusal"].to_string());
                }
            }
        }
    }

    ContentType::Incomplete
}

pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}

fn deserialize_success(response: &SuccessResponse) -> anyhow::Result<OpenAIResults> {
    let text = match extract_completed_object(response) {
        ContentType::Text(text) => text,
        ContentType::Refusal(refusal) => bail!("OpenAI returned a refusal: {refusal}"),
        ContentType::Incomplete => bail!("Query never completed"),
    };

    let structured_output: StructuredOutput =
        serde_json::from_str(&text).context("Failed to deserialize structured output")?;

    let results = OpenAIResults {
        input_tokens: response.usage.input_tokens,
        output_tokens: response.usage.output_tokens,
        code: structured_output.code,
        description_of_what_was_done: structured_output.description_of_what_was_done,
    };
    Ok(results)
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<OpenAIResults> {
    let response: RawResponse =
        serde_json::from_str(&raw_json).context("Failed to deserialize raw JSON")?;

    match response {
        RawResponse::Success(response) => deserialize_success(&response),
        RawResponse::Error(error) => bail!(error.error.message),
    }
}
