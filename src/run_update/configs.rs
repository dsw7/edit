use std::path::PathBuf;

use crate::cli::UpdateConfigs;
use crate::core::ConfigsFromFile;

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

impl Configs {
    pub fn override_from_cli(mut self, configs_from_cli: UpdateConfigs) -> Self {
        self.code_to_edit = configs_from_cli.code_to_edit;
        self.echo = configs_from_cli.echo;
        self.filename = configs_from_cli.filename;
        self.pretty = configs_from_cli.pretty;
        self.prompt = configs_from_cli.instructions;
        self
    }

    pub fn override_from_file(mut self, configs_from_file: ConfigsFromFile) -> Self {
        self.model = configs_from_file.anthropic.model;
        self.max_tokens = configs_from_file.anthropic.max_tokens.clamp(1, 64000);
        self
    }

    pub fn override_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }
}
