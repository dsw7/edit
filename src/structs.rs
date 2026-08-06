use std::path::PathBuf;

pub struct CliParameters {
    pub input_file: PathBuf,
    pub model: String,
    pub prompt: Option<String>,
}

pub struct OpenAIParams {
    pub prompt: String,
    pub model: String,
}

#[derive(Debug)]
pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}
