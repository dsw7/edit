use std::env;
use std::path::PathBuf;

use anyhow::Context;

use super::config_file::read_configs_from_file;
use crate::cli::UpdateConfigs;

#[derive(Debug, Default)]
pub struct Configs {
    pub api_key: String,
    pub code_to_edit: String,
    pub echo: bool,
    pub filename: PathBuf,
    pub max_tokens: u16,
    pub model: String,
    pub pretty: bool,
    pub prompt: String,
}

impl From<UpdateConfigs> for Configs {
    fn from(cli: UpdateConfigs) -> Self {
        Self {
            code_to_edit: cli.code_to_edit,
            echo: cli.echo,
            filename: cli.filename,
            pretty: cli.pretty,
            prompt: cli.instructions,
            ..Default::default()
        }
    }
}

impl Configs {
    fn try_load_anthropic_api_key(mut self) -> anyhow::Result<Self> {
        self.api_key = env::var("ANTHROPIC_API_KEY").context("failed to load Anthropic API key")?;
        Ok(self)
    }

    fn try_load_configs_from_file(mut self) -> anyhow::Result<Self> {
        let configs = read_configs_from_file()?;
        self.model = configs.anthropic.model;
        self.max_tokens = configs.anthropic.max_tokens.clamp(1, 64000);
        Ok(self)
    }
}

pub fn setup_configurations(run_configs: UpdateConfigs) -> anyhow::Result<Configs> {
    let configs = Configs::from(run_configs)
        .try_load_anthropic_api_key()?
        .try_load_configs_from_file()?;

    Ok(configs)
}
