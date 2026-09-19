use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ApiResponse {
    ErrorResponse(ResponseError),
    SuccessResponse(Response),
}

#[derive(Deserialize, Debug)]
struct ResponseError {
    error: Error,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    content: Vec<ContentBlock>,
    usage: Usage,

    // see https://platform.claude.com/docs/en/api/http/messages#stop_reason
    stop_reason: String,
}

#[derive(Deserialize, Debug)]
struct ContentBlock {
    // see https://platform.claude.com/docs/en/api/http/messages#content_block
    #[serde(rename = "type")]
    content_type: String,

    text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Usage {
    // see https://platform.claude.com/docs/en/api/http/messages#usage
    input_tokens: u32,
    output_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

fn unpack_structured_output(text: &str) -> anyhow::Result<StructuredOutput> {
    let structured_output = serde_json::from_str::<StructuredOutput>(text)
        .context("failed to deserialize structured output")?;

    Ok(structured_output)
}

fn unpack_text_block(response: &Response) -> anyhow::Result<StructuredOutput> {
    if response.content.is_empty() {
        anyhow::bail!("content array is empty");
    }

    for content_block in &response.content {
        // see https://platform.claude.com/docs/en/api/http/messages#text_block
        if content_block.content_type == "text" {
            if let Some(text) = &content_block.text {
                return unpack_structured_output(text);
            }
        }
    }

    anyhow::bail!("no TextBlock object found in response")
}

#[derive(Debug)]
pub struct AnthropicResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}

fn unpack_response(response: &Response) -> anyhow::Result<AnthropicResults> {
    if response.stop_reason != "end_turn" {
        anyhow::bail!(format!(
            "query stopped prematurely: {}",
            response.stop_reason
        ))
    }

    let structured_output = unpack_text_block(response)?;
    //.context("something went wrong when unpacking structured output")?;

    let results = AnthropicResults {
        input_tokens: response.usage.input_tokens,
        output_tokens: response.usage.output_tokens,
        code: structured_output.code,
        description_of_what_was_done: structured_output.description_of_what_was_done,
    };

    Ok(results)
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<AnthropicResults> {
    let response = serde_json::from_str::<ApiResponse>(&raw_json)
        .context("failed to deserialize raw json from Anthropic")?;

    match response {
        ApiResponse::ErrorResponse(response) => anyhow::bail!(response.error.message),
        ApiResponse::SuccessResponse(response) => unpack_response(&response),
    }
}

#[cfg(test)]
mod tests {
    use super::deserialize_json_response;

    fn assert_error_message(raw_json: &str, expected_error: &str) {
        let result = deserialize_json_response(raw_json.to_string());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), expected_error);
    }

    #[test]
    fn test_deserialize_broken_json() {
        let raw_json = r#"{
            "error": {
        }"#;
        assert_error_message(raw_json, "failed to deserialize raw json from Anthropic");
    }

    #[test]
    fn test_deserialize_error_response() {
        let raw_json = r#"{
            "error": {"message": "Something went wrong"}
        }"#;
        assert_error_message(raw_json, "Something went wrong");
    }

    #[test]
    fn test_deserialize_success_response() {
        let raw_json = r#"{
            "content": [
                {
                    "type": "text",
                    "text": "{\"code\": \"print('Hello, world!')\", \"description_of_what_was_done\": \"A simple hello world code\"}"
                }
            ],
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        }"#;

        let response = deserialize_json_response(raw_json.to_string()).unwrap();
        assert_eq!(response.input_tokens, 100);
        assert_eq!(response.output_tokens, 50);
        assert_eq!(response.code, "print('Hello, world!')");
        assert_eq!(
            response.description_of_what_was_done,
            "A simple hello world code"
        );
    }

    #[test]
    fn test_deserialize_success_response_invalid_stop_reason() {
        let raw_json = r#"{
            "content": [
                { "type": "text", "text": "{\"code\": " }
            ],
            "stop_reason": "max_tokens",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        }"#;
        assert_error_message(raw_json, "query stopped prematurely: max_tokens");
    }

    #[test]
    fn test_deserialize_success_response_empty_content() {
        let raw_json = r#"{
            "content": [],
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        }"#;
        assert_error_message(raw_json, "content array is empty");
    }

    #[test]
    fn test_deserialize_success_response_completed_not_found() {
        let raw_json = r#"{
            "content": [
                { "type": "tool_use", "name": "code_edit_tool" }
            ],
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        }"#;
        assert_error_message(raw_json, "no TextBlock object found in response");
    }

    #[test]
    fn test_deserialize_success_response_invalid_structured_output() {
        let raw_json = r#"{
            "content": [
                { "type": "tool_use", "name": "code_edit_tool" },
                {
                    "type": "text",
                    "text": "{\"code\": \"print('Hello, world!')\"}"
                }
            ],
            "stop_reason": "end_turn",
            "usage": { "input_tokens": 100, "output_tokens": 50 }
        }"#;
        assert_error_message(raw_json, "failed to deserialize structured output");
    }
}
