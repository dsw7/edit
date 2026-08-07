pub struct OpenAIParams {
    pub model: String,
    pub prompt: String,
}

impl OpenAIParams {
    #[cfg(test)]
    pub fn from_str(model: &str, prompt: &str) -> Self {
        OpenAIParams {
            model: model.to_string(),
            prompt: prompt.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}
