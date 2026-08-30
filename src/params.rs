use std::path::PathBuf;

pub struct Parameters {
    pub enable_prompt_validation: bool,
    pub input_file: PathBuf,
    pub model: String,
    pub provider: String,

    // Validation
    pub ollama_host: String,
    pub ollama_port: u16,
    pub ollama_validation_model: String,
}
