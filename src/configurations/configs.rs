use std::fs;
use std::path::PathBuf;

use anyhow::Context;

use super::configs_from_file::ConfigsFromFile;
use crate::program_files;

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
}

pub fn load_configs() -> anyhow::Result<ConfigsFromFile> {
    let app_dir = program_files::get_app_dir()?;
    let config_file = program_files::get_config_file(&app_dir);

    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}
