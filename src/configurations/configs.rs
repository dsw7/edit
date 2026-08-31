use std::fs;

use anyhow::Context;
use serde::{Deserialize, Deserializer};

use crate::program_files;

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
pub struct Configs {
    #[serde(deserialize_with = "check_not_empty")]
    pub provider: String,

    pub disable_prompt_validation: bool,

    pub ollama: Ollama,
    pub openai: OpenAI,
}

#[derive(Deserialize, Debug)]
pub struct Ollama {
    pub ollama_port: u16,
    pub validation_context_window: u16,

    #[serde(deserialize_with = "check_not_empty")]
    pub ollama_host: String,

    #[serde(deserialize_with = "check_not_empty")]
    pub ollama_validation_model: String,
}

#[derive(Deserialize, Debug)]
pub struct OpenAI {
    #[serde(deserialize_with = "check_not_empty")]
    pub code_edit_model: String,
}

pub fn load_configs() -> anyhow::Result<Configs> {
    let app_dir = program_files::get_app_dir()?;
    let config_file = program_files::get_config_file(&app_dir);

    let toml_str = fs::read_to_string(&config_file)
        .context(format!("cannot read {}", config_file.display()))?;

    let configs = toml::from_str::<Configs>(&toml_str)
        .context(format!("failed to parse {}", config_file.display()))?;

    Ok(configs)
}
