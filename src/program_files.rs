use std::env;
use std::path::{Path, PathBuf};

fn get_home_dir() -> anyhow::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => anyhow::bail!("couldn't get home directory"),
    }
}

pub fn get_app_dir() -> anyhow::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    Ok(home_dir.join(".edit"))
}

pub fn get_config_file(app_dir: &Path) -> PathBuf {
    app_dir.join("config.toml")
}
