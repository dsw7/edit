use serde::{Deserialize, Deserializer};

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
