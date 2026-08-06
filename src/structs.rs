use std::path::PathBuf;

pub struct Parameters {
    pub input_file: PathBuf,
    pub model: String,
    pub prompt: Option<String>,
}

#[derive(Debug)]
pub struct OpenAIResults {
    pub input_tokens: u32,
    pub output_tokens: u32,
    pub code: String,
    pub description_of_what_was_done: String,
}
