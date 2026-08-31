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

pub fn setup_configurations() -> anyhow::Result<Configs> {
    let cfgs_cli = ConfigsFromCli::parse();
    let cfgs_file = load_configs_from_file()?;

    let provider = match cfgs_cli.provider {
        Some(provider) => provider,
        None => cfgs_file.provider,
    };

    let code_edit_model = match provider.as_str() {
        "openai" => cfgs_file.openai.code_edit_model,
        _ => anyhow::bail!(format!("invalid provider: `{provider}`")),
    };

    let params = Configs {
        disable_prompt_validation: cfgs_file.disable_prompt_validation,
        input_file: cfgs_cli.file_to_edit,
        code_edit_model,
        provider,
        ollama_host: cfgs_file.ollama.ollama_host,
        ollama_port: cfgs_file.ollama.ollama_port,
        ollama_validation_model: cfgs_file.ollama.ollama_validation_model,
        validation_context_window: cfgs_file.ollama.validation_context_window,
    };

    Ok(params)
}
