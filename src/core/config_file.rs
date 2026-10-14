use std::fs;
use std::path::PathBuf;

use anyhow::Context;
use serde::{Deserialize, Deserializer, Serialize};

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

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ConfigsFromFile {
    pub anthropic: Anthropic,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Anthropic {
    #[serde(deserialize_with = "check_not_empty")]
    pub model: String,

    pub max_tokens: u16,
}

impl Default for Anthropic {
    fn default() -> Self {
        Self {
            model: String::from("claude-haiku-4-5"),
            max_tokens: 4096,
        }
    }
}

pub fn write_configs_to_file(config_file: &PathBuf) -> anyhow::Result<()> {
    let configs = ConfigsFromFile::default();
    let toml_str = toml::to_string_pretty(&configs)?;

    fs::write(config_file, toml_str)?;
    Ok(())
}

pub fn read_configs_from_file(config_file: &PathBuf) -> anyhow::Result<ConfigsFromFile> {
    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<ConfigsFromFile>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}
