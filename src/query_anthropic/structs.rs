#[derive(Debug)]
pub struct AnthropicResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}
