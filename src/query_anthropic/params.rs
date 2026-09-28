#[derive(Debug)]
pub struct AnthropicParams {
    pub max_tokens: u16,
    pub model: String,
}

impl Default for AnthropicParams {
    fn default() -> Self {
        Self {
            max_tokens: 4096,
            model: String::from("claude-haiku-4-5"),
        }
    }
}
