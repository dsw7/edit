use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use super::configs_from_cli::ConfigsFromCli;
use super::configs_from_file::ConfigsFromFile;

use crate::program_files;

pub struct Configs {
    // CLI
    pub code_to_edit: String,
    pub filename: PathBuf,
    pub instructions: String,

    // code editing
    pub code_edit_model: String,
    pub max_tokens_edit_model: u16,
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

    let max_tokens_edit_model = cfgs_file.anthropic.max_tokens_edit_model.clamp(1, 64000);

    Ok(Configs {
        code_edit_model: cfgs_file.anthropic.code_edit_model,
        code_to_edit: cfgs_cli.code_to_edit,
        filename: cfgs_cli.filename,
        instructions: cfgs_cli.instructions,
        max_tokens_edit_model,
    })
}
