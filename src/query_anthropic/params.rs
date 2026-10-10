#[derive(Debug)]
pub struct AnthropicParams {
    pub code_to_edit: String,
    pub max_tokens: u16,
    pub model: String,
    pub programming_language: String,
    pub prompt: String,
}

impl Default for AnthropicParams {
    fn default() -> Self {
        Self {
            code_to_edit: String::from(""),
            max_tokens: 4096,
            model: String::from("claude-haiku-4-5"),
            programming_language: String::from("Python"),
            prompt: String::from(""),
        }
    }
}
