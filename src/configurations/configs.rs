use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use clap::Parser;

use super::configs_from_cli::ConfigsFromCli;
use super::configs_from_file::ConfigsFromFile;
use super::resolve_programming_language::resolve_lang_from_extension;

use crate::program_files;

#[derive(Debug, Default)]
pub struct Configs {
    pub code_to_edit: String,
    pub lang: String,
    pub max_tokens: u16,
    pub model: String,
    pub prompt: String,

    filename: PathBuf,
}

impl Configs {
    fn load_configs_from_cli(mut self) -> Self {
        let configs = ConfigsFromCli::parse();
        self.code_to_edit = configs.code_to_edit;
        self.filename = configs.filename;
        self.prompt = configs.instructions;
        self
    }

    fn load_configs_from_file(mut self) -> anyhow::Result<Self> {
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

    fn resolve_lang_metadata(mut self) -> anyhow::Result<Self> {
        self.lang = resolve_lang_from_extension(&self.filename)?;
        Ok(self)
    }
}

pub fn setup_configurations() -> anyhow::Result<Configs> {
    let configs = Configs::default()
        .load_configs_from_cli()
        .load_configs_from_file()?
        .resolve_lang_metadata()?;

    Ok(configs)
}
