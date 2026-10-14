use std::fs;

use anyhow::Context;
use serde::{Deserialize, Deserializer};

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

#[derive(Deserialize, Debug)]
pub struct ConfigsFromFile {
    pub anthropic: Anthropic,
}

#[derive(Deserialize, Debug)]
pub struct Anthropic {
    #[serde(deserialize_with = "check_not_empty")]
    pub model: String,

    pub max_tokens: u16,
}

pub fn read_configs() -> anyhow::Result<ConfigsFromFile> {
    let app_dir = app_dir::get_app_dir()?;
    let config_file = app_dir.join("config.toml");

    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}
