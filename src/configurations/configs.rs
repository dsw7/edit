use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use super::configs_from_cli::ConfigsFromCli;
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

fn load_configs_from_file() -> anyhow::Result<ConfigsFromFile> {
    let app_dir = program_files::get_app_dir()?;
    let config_file = program_files::get_config_file(&app_dir);

    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}

pub fn load_configs() -> anyhow::Result<Configs> {
    let cli = ConfigsFromCli::parse();
    let configs = load_configs_from_file()?;

    let provider = match cli.provider {
        Some(provider) => provider,
        None => configs.provider,
    };

    let code_edit_model = match provider.as_str() {
        "openai" => configs.openai.code_edit_model,
        _ => anyhow::bail!(format!("invalid provider: `{provider}`")),
    };

    let params = Configs {
        disable_prompt_validation: configs.disable_prompt_validation,
        input_file: cli.file_to_edit,
        code_edit_model,
        provider,
        ollama_host: configs.ollama.ollama_host,
        ollama_port: configs.ollama.ollama_port,
        ollama_validation_model: configs.ollama.ollama_validation_model,
        validation_context_window: configs.ollama.validation_context_window,
    };

    Ok(params)
}
