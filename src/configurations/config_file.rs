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
    pub anthropic: Anthropic,
}

#[derive(Deserialize, Debug)]
pub struct Anthropic {
    #[serde(deserialize_with = "check_not_empty")]
    pub code_edit_model: String,

    pub max_tokens_edit_model: u16,
}
