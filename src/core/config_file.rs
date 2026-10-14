use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Deserializer, Serialize};

use super::app_dir;

fn check_not_empty<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;

    if value.is_empty() {
        Err(serde::de::Error::custom("string cannot be empty"))
    } else {
        Ok(value)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigsFromFile {
    pub anthropic: Anthropic,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Anthropic {
    #[serde(deserialize_with = "check_not_empty")]
    pub model: String,

    pub max_tokens: u16,
}

fn get_path_configs_file() -> anyhow::Result<PathBuf> {
    let app_dir = app_dir::get_app_dir()?;
    Ok(app_dir.join("config.toml"))
}

pub fn read_configs_from_file() -> anyhow::Result<ConfigsFromFile> {
    let config_file = get_path_configs_file()?;

    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}

pub fn write_configs_to_file(configs: &ConfigsFromFile) -> anyhow::Result<()> {
    let config_file = get_path_configs_file()?;

    let toml_str = toml::to_string_pretty(configs)?;
    fs::write(config_file, toml_str)?;
    Ok(())
}
