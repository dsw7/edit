use std::path::PathBuf;

pub struct CliParameters {
    pub input_file: PathBuf,
    pub model: String,
    pub prompt: Option<String>,
}
