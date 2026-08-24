use anyhow::Context;
use serde::Deserialize;

use super::structs::OpenAIResults;

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
    // ResponseStatus string may or may not exist
    status: Option<String>,

    // will always exist as object or null
    incomplete_details: Option<IncompleteDetails>,

    // will always exist
    output: Vec<ResponseOutputMessage>,

    // ResponseUsage object may or may not exist - populate with defaults
    #[serde(default = "default_usage")]
    usage: ResponseUsage,
}

#[derive(Deserialize, Debug)]
struct IncompleteDetails {
    reason: String,
}

fn unpack_incomplete_details(response: &Response) -> String {
    match &response.incomplete_details {
        Some(details) => format!("query incomplete: {}", details.reason),
        None => String::from("query incomplete: no details provided"),
    }
}

#[derive(Deserialize, Debug)]
struct ResponseOutputMessage {
    status: String,
    content: Vec<TextOrRefusal>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum TextOrRefusal {
    Text(ResponseOutputText),
    Refusal(ResponseOutputRefusal),
}

#[derive(Deserialize, Debug)]
struct ResponseOutputText {
    text: String,
}

#[derive(Deserialize, Debug)]
struct ResponseOutputRefusal {
    refusal: String,
}

fn unpack_output(response: &Response) -> anyhow::Result<StructuredOutput> {
    if response.output.is_empty() {
        anyhow::bail!("output array is empty");
    }

    for object in &response.output {
        if object.status == "completed" {
            return unpack_content(&object.content);
        }
    }

    anyhow::bail!("query completed but no completed message found")
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

fn unpack_content(content: &[TextOrRefusal]) -> anyhow::Result<StructuredOutput> {
    if content.is_empty() {
        anyhow::bail!("content array is empty");
    }

    let raw_text = match &content[0] {
        TextOrRefusal::Text(text) => &text.text,
        TextOrRefusal::Refusal(refusal) => anyhow::bail!("query refused: {}", refusal.refusal),
    };

    let structured_output = serde_json::from_str::<StructuredOutput>(raw_text)
        .context("failed to deserialize structured output")?;

    Ok(structured_output)
}

#[derive(Deserialize, Debug)]
struct ResponseUsage {
    input_tokens: u32,
    output_tokens: u32,
}

fn default_usage() -> ResponseUsage {
    ResponseUsage {
        input_tokens: 0,
        output_tokens: 0,
    }
}

fn unpack_response(response: &Response) -> anyhow::Result<OpenAIResults> {
    let status = match &response.status {
        Some(status) => status,
        None => anyhow::bail!("no status could be found in response"),
    };

    let structured_output = match status.as_str() {
        "completed" => unpack_output(response)?,
        "incomplete" => anyhow::bail!(unpack_incomplete_details(response)),
        _ => anyhow::bail!(format!("query did not finish: {status}")),
    };

    let results = OpenAIResults {
        input_tokens: response.usage.input_tokens,
        output_tokens: response.usage.output_tokens,
        code: structured_output.code,
        description_of_what_was_done: structured_output.description_of_what_was_done,
    };

    Ok(results)
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<OpenAIResults> {
    let response =
        serde_json::from_str::<ApiResponse>(&raw_json).context("failed to deserialize raw json")?;

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
    fn test_deserialize_error_response() {
        let raw_json = r#"{
            "error": {"message": "Model not found"}
        }"#;
        assert_error_message(raw_json, "Model not found");
    }

    #[test]
    fn test_deserialize_success_response_missing_status() {
        let raw_json = r#"{
            "output": []
        }"#;
        assert_error_message(raw_json, "no status could be found in response");
    }

    #[test]
    fn test_deserialize_success_response_never_completed() {
        let raw_json = r#"{
            "status": "in_progress",
            "output": [{
                "status": "in_progress",
                "content": []
            }]
        }"#;
        assert_error_message(raw_json, "query did not finish: in_progress");
    }

    #[test]
    fn test_deserialize_success_response_no_output() {
        let raw_json = r#"{
            "status": "completed",
            "output": []
        }"#;
        assert_error_message(raw_json, "output array is empty");
    }

    #[test]
    fn test_deserialize_success_response_completed_not_found() {
        let raw_json = r#"{
            "status": "completed",
            "output": [{
                "status": "",
                "content": []
            }]
        }"#;
        assert_error_message(raw_json, "query completed but no completed message found");
    }

    #[test]
    fn test_deserialize_success_response_empty_content() {
        let raw_json = r#"{
            "status": "completed",
            "output": [{
                "status": "completed",
                "content": []
            }]
        }"#;
        assert_error_message(raw_json, "content array is empty");
    }

    #[test]
    fn test_deserialize_success_response_refusal() {
        let raw_json = r#"{
            "status": "completed",
            "output": [{
                "status": "completed",
                "content": [{
                    "refusal": "The query was too long"
                }]
            }]
        }"#;
        assert_error_message(raw_json, "query refused: The query was too long");
    }

    #[test]
    fn test_deserialize_success_response_incomplete_no_details() {
        let raw_json = r#"{
            "status": "incomplete",
            "output": []
        }"#;
        assert_error_message(raw_json, "query incomplete: no details provided");
    }

    #[test]
    fn test_deserialize_success_response_incomplete() {
        let raw_json = r#"{
            "status": "incomplete",
            "incomplete_details": {
                "reason": "max_output_tokens"
            },
            "output": []
        }"#;
        assert_error_message(raw_json, "query incomplete: max_output_tokens");
    }

    #[test]
    fn test_deserialize_success_response_output_text() {
        let raw_json = r#"{
            "status": "completed",
            "output": [{
                "status": "completed",
                "content": [{
                    "text": "{\"code\": \"print('Hello, world!')\", \"description_of_what_was_done\": \"A simple hello world code\"}"
                }]
            }],
            "usage": {"input_tokens": 100, "output_tokens": 50}
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
    fn test_deserialize_success_response_output_text_default_usage() {
        let raw_json = r#"{
            "status": "completed",
            "output": [{
                "status": "completed",
                "content": [{
                    "text": "{\"code\": \"print('Hello, world!')\", \"description_of_what_was_done\": \"A simple hello world code\"}"
                }]
            }]
        }"#;

        let response = deserialize_json_response(raw_json.to_string()).unwrap();
        assert_eq!(response.input_tokens, 0);
        assert_eq!(response.output_tokens, 0);
    }
}
