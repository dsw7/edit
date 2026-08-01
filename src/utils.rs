use std::env;

use anyhow::Context;

pub fn load_api_key(env_var_key: &str) -> anyhow::Result<String> {
    let env_var_value = env::var(env_var_key).context(format!(
        "Failed to load environment variable: {env_var_key}"
    ))?;

    Ok(env_var_value)
}
