use anyhow::Context;
use serde::Deserialize;

use super::structs::ValidationResults;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ApiResponse {
    ErrorResponse(Error),
    SuccessResponse(Response),
}

#[derive(Deserialize, Debug)]
struct Error {
    error: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    eval_count: u32,
    prompt_eval_count: u32,
    response: String,
    total_duration: u32,
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    reasoning: String,
    valid_instructions: bool,
}

fn nanoseconds_to_seconds(ns: u32) -> f32 {
    ns as f32 / 1_000_000_000.0
}

fn unpack_response(response: &Response) -> anyhow::Result<ValidationResults> {
    let structured_output = serde_json::from_str::<StructuredOutput>(&response.response)
        .context("failed to deserialize structured output")?;

    let results = ValidationResults {
        total_duration: nanoseconds_to_seconds(response.total_duration),
        reasoning: structured_output.reasoning,
        valid_instructions: structured_output.valid_instructions,
        input_tokens: response.prompt_eval_count,
        output_tokens: response.eval_count,
    };

    Ok(results)
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<ValidationResults> {
    let response =
        serde_json::from_str::<ApiResponse>(&raw_json).context("failed to deserialize raw json")?;

    match response {
        ApiResponse::ErrorResponse(response) => anyhow::bail!(response.error),
        ApiResponse::SuccessResponse(response) => unpack_response(&response),
    }
}
