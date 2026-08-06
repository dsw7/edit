use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum ApiResponse {
    ErrorResponse(ErrorResponse),
    SuccessResponse(Response),
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
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

    // will always exist as ResponseError object or null
    error: Option<ResponseError>,

    // will always exist as object or null
    incomplete_details: Option<IncompleteDetails>,

    // will always exist
    output: Vec<ResponseOutputMessage>,

    // ResponseUsage object may or may not exist - populate with defaults
    #[serde(default = "default_usage")]
    usage: ResponseUsage,
}

#[derive(Deserialize, Debug)]
struct ResponseError {
    code: String,
    message: String,
}

fn unpack_error(response: &Response) -> anyhow::Result<()> {
    match &response.error {
        Some(details) => anyhow::bail!(
            "Query failed with code `{}` and message `{}`",
            details.code,
            details.message
        ),
        None => anyhow::bail!("Query failed. No details provided"),
    }
}

#[derive(Deserialize, Debug)]
struct IncompleteDetails {
    reason: String,
}

fn unpack_incomplete_details(response: &Response) -> anyhow::Result<()> {
    match &response.incomplete_details {
        Some(details) => anyhow::bail!("Query incomplete: {}", details.reason),
        None => anyhow::bail!("Query incomplete. No details provided"),
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

fn unpack_output(response: &Response) -> anyhow::Result<()> {
    for object in &response.output {
        if object.status == "completed" {
            return unpack_content(&object.content);
        }
    }

    anyhow::bail!("Query completed but no completed message found!")
}

fn unpack_content(content: &[TextOrRefusal]) -> anyhow::Result<()> {
    match &content[0] {
        TextOrRefusal::Text(text) => println!("{}", text.text),
        TextOrRefusal::Refusal(refusal) => anyhow::bail!("Query refused: {}", refusal.refusal),
    }

    Ok(())
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

fn unpack_response(response: &Response) -> anyhow::Result<()> {
    match &response.status {
        Some(status) => match status.as_str() {
            "completed" => unpack_output(response),
            "incomplete" => unpack_incomplete_details(response),
            "failed" => unpack_error(response),
            _ => anyhow::bail!(format!("Query did not finish. Status: {status}")),
        },
        None => anyhow::bail!("No status could be found in response"),
    }
}

pub fn deserialize_json_response(raw_json: String) -> anyhow::Result<()> {
    let response =
        serde_json::from_str::<ApiResponse>(&raw_json).context("Failed to deserialize raw JSON")?;

    match response {
        ApiResponse::ErrorResponse(error) => anyhow::bail!(error.error.message),
        ApiResponse::SuccessResponse(response) => unpack_response(&response),
    }
}

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
    response
        .output
        .iter()
        .filter(|object| object.status == "completed")
        .flat_map(|object| &object.content)
        .filter_map(|content| {
            content
                .get("type")
                .and_then(|v| v.as_str())
                .map(|content_type| (content_type, content))
        })
        .find_map(|(content_type, content)| match content_type {
            "output_text" => content
                .get("text")
                .and_then(|v| v.as_str())
                .map(|text| ContentType::Text(text.to_string())),
            "refusal" => content
                .get("refusal")
                .and_then(|v| v.as_str())
                .map(|refusal| ContentType::Refusal(refusal.to_string())),
            _ => None,
        })
        .unwrap_or(ContentType::Incomplete)
}

#[derive(Deserialize, Debug)]
struct StructuredOutput {
    code: String,
    description_of_what_was_done: String,
}

#[derive(Debug)]
pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}

fn deserialize_success(response: &SuccessResponse) -> anyhow::Result<OpenAIResults> {
    let text = match extract_completed_object(response) {
        ContentType::Text(text) => text,
        ContentType::Refusal(refusal) => anyhow::bail!("OpenAI returned a refusal: {refusal}"),
        ContentType::Incomplete => anyhow::bail!("Query never completed"),
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
        RawResponse::Error(error) => anyhow::bail!(error.error.message),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_error_response() {
        let raw_json = r#"{
            "error": {"message": "Model not found"}
        }"#;

        let result = deserialize_json_response(raw_json.to_string());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Model not found");
    }

    #[test]
    fn test_deserialize_success_response_output_text() {
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

    #[test]
    fn test_deserialize_success_response_refusal() {
        let raw_json = r#"{
            "usage": {"input_tokens": 100, "output_tokens": 50},
            "output": [{
                "status": "completed",
                "content": [{
                    "type": "refusal",
                    "refusal": "The query was too long"
                }]
            }]
        }"#;

        let result = deserialize_json_response(raw_json.to_string());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "OpenAI returned a refusal: The query was too long"
        );
    }

    #[test]
    fn test_deserialize_success_response_never_completed() {
        let raw_json = r#"{
            "usage": {"input_tokens": 100, "output_tokens": 50},
            "output": [{
                "status": "in progress",
                "content": []
            }]
        }"#;

        let result = deserialize_json_response(raw_json.to_string());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Query never completed");
    }
}
