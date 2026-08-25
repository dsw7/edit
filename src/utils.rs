use std::env;
use std::fs;
use std::path;

use anyhow::Context;

pub fn load_api_key(env_var_key: &str) -> anyhow::Result<String> {
    let env_var_value = env::var(env_var_key).context(format!(
        "failed to load environment variable: {env_var_key}"
    ))?;

    Ok(env_var_value)
}

pub fn read_file(file: &path::Path) -> anyhow::Result<String> {
    fs::read_to_string(file).context(format!("failed to read file `{}`", &file.display()))
}

pub fn write_to_file(file: &path::Path, contents: &str) -> anyhow::Result<()> {
    fs::write(file, contents).context(format!("failed to write to file `{}`", &file.display()))
}
