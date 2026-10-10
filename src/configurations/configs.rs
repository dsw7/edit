use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use super::configs_from_cli::ConfigsFromCli;
use super::configs_from_file::ConfigsFromFile;

use crate::program_files;

#[derive(Debug, Default)]
pub struct Configs {
    pub api_key: String,
    pub code_to_edit: String,
    pub filename: PathBuf,
    pub max_tokens: u16,
    pub model: String,
    pub prompt: String,
}

impl Configs {
    fn load_configs_from_cli(mut self) -> Self {
        let configs = ConfigsFromCli::parse();
        self.code_to_edit = configs.code_to_edit;
        self.filename = configs.filename;
        self.prompt = configs.instructions;
        self
    }

    fn try_load_anthropic_api_key(mut self) -> anyhow::Result<Self> {
        let key_name = "ANTHROPIC_API_KEY";
        self.api_key =
            env::var(key_name).context("failed to load environment variable: {key_name}")?;

        Ok(self)
    }

    fn try_load_configs_from_file(mut self) -> anyhow::Result<Self> {
        let app_dir = program_files::get_app_dir()?;
        let config_file = program_files::get_config_file(&app_dir);

        let toml_str = fs::read_to_string(&config_file)
            .context(format!("cannot read {}", config_file.display()))?;

        let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
            .context(format!("failed to parse {}", config_file.display()))?;

        self.model = configs.anthropic.code_edit_model;
        self.max_tokens = configs.anthropic.max_tokens_edit_model.clamp(1, 64000);
        Ok(self)
    }
}

pub fn setup_configurations() -> anyhow::Result<Configs> {
    let configs = Configs::default()
        .load_configs_from_cli()
        .try_load_anthropic_api_key()?
        .try_load_configs_from_file()?;

    Ok(configs)
}
