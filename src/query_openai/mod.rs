mod queries;
mod response;
mod structs;

pub use queries::{edit_code_block, write_new_code};
pub use structs::OpenAIResults;

#[cfg(test)]
mod tests {
    use super::write_new_code;

    fn assert_error_message(model: &str, prompt: &str, expected_error: &str) {
        let result = write_new_code(model.to_string(), prompt.to_string());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(error.to_string(), expected_error);
    }

    #[test]
    fn test_invalid_model() {
        assert_error_message(
            "foobar",
            "What is 3 + 5?",
            "The requested model 'foobar' does not exist.",
        );
    }

    #[test]
    fn test_incompatible_model() {
        assert_error_message(
            "gpt-3.5-turbo",
            "What is 3 + 5?",
            "Invalid parameter: 'text.format' of type 'json_schema' is not supported with model version `gpt-3.5-turbo`.",
        );
    }

    #[test]
    fn test_valid_query() {
        let model = String::from("gpt-4o");
        let prompt = String::from("Print 'hello world' in Python.");
        let result = write_new_code(model, prompt).unwrap();
        assert!(result.input_tokens > 0);
        assert!(result.output_tokens > 0);
        assert!(!result.description_of_what_was_done.is_empty());
        assert_eq!(result.code, "print('hello world')");
    }
}
