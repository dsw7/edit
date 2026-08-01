use anyhow::{Context, bail};
use serde::Deserialize;

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
    content: Vec<TextOrRefusal>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TextOrRefusal {
    TextResponse { text: String },
    RefusalResponse { refusal: String },
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

// structured output

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

fn extract_output_text(response: &SuccessResponse) -> anyhow::Result<String> {
    for object in &response.output {
        if object.status == "completed" {
            return match &object.content[0] {
                TextOrRefusal::TextResponse { text } => Ok(text.clone()),
                TextOrRefusal::RefusalResponse { refusal } => bail!(refusal.clone()),
            };
        }
    }

    bail!("Query never completed")
}

pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}

fn deserialize_success(response: &SuccessResponse) -> anyhow::Result<OpenAIResults> {
    let text = extract_output_text(&response)?;
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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<OpenAIResults> {
    let response: RawResponse =
        serde_json::from_str(&raw_json).context("Failed to deserialize raw JSON")?;

    match response {
        RawResponse::Success(response) => deserialize_success(&response),
        RawResponse::Error(error) => bail!(error.error.message),
    }
}
