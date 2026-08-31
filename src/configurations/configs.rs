use std::path::PathBuf;

use super::load_configs_from_file::ConfigsFromFile;

pub struct Configs {
    // CLI
    pub input_file: PathBuf,
    pub provider: String,

    // code editing
    pub code_edit_model: String,

    // validation
    pub disable_prompt_validation: bool,
    pub ollama_host: String,
    pub ollama_port: u16,
    pub ollama_validation_model: String,
    pub validation_context_window: u16,
    //configs: ConfigsFromFile,
}
