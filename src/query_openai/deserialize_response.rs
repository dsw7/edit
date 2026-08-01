use anyhow::{Context, bail};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum RawResponse {
    Success(SuccessResponse),
    Error(ErrorResponse),
}

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

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    error: Error,
}

#[derive(Deserialize, Debug)]
struct Error {
    message: String,
}

enum ContentType {
    Text(String),
    Refusal(String),
    Incomplete,
}

fn extract_completed_object(response: &SuccessResponse) -> ContentType {
    for object in &response.output {
        if object.status == "completed" {
            for content in &object.content {
                if let Some(content_type) = content.get("type").and_then(|v| v.as_str()) {
                    if content_type == "output_text" {
                        if let Some(text) = content.get("text").and_then(|v| v.as_str()) {
                            return ContentType::Text(text.to_string());
                        }
                    } else if content_type == "refusal" {
                        if let Some(refusal) = content.get("refusal").and_then(|v| v.as_str()) {
                            return ContentType::Refusal(refusal.to_string());
                        }
                    }
                }
            }
        }
    }

    ContentType::Incomplete
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_success_response() {
        let raw_json = r#"{
            "usage": {"input_tokens": 100, "output_tokens": 50},
            "output": [{
                "status": "completed",
                "content": [{
                    "type": "output_text",
                    "text": "{\"code\": \"print('Hello, world!')\", \"description_of_what_was_done\": \"A simple hello world code\"}"
                }]
            }]
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

    /*
    #[test]
    fn test_deserialize_error_response() {
        let raw_json = r#"{
            "error": {"message": "An error occurred"}
        }"#;

        let result = deserialize_json_response(raw_json.to_string());

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "An error occurred");
    }
    */
}
