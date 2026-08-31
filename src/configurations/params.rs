use std::path::PathBuf;

pub struct Parameters {
    pub code_edit_model: String,
    pub disable_prompt_validation: bool,
    pub input_file: PathBuf,
    pub provider: String,

    // Validation
    pub ollama_host: String,
    pub ollama_port: u16,
    pub ollama_validation_model: String,
    pub validation_context_window: u16,
}
