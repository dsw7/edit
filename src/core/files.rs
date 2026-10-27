use std::env;
use std::path::PathBuf;

fn try_home_dir() -> anyhow::Result<PathBuf> {
    match env::home_dir() {
        Some(path) => Ok(path),
        None => anyhow::bail!("couldn't get home directory"),
    }
}

pub fn try_app_dir() -> anyhow::Result<PathBuf> {
    let home_dir = try_home_dir()?;
    Ok(home_dir.join(".edit"))
}

pub fn try_config_file() -> anyhow::Result<PathBuf> {
    let app_dir = try_app_dir()?;
    Ok(app_dir.join("config.toml"))
}

pub fn try_log_dir() -> anyhow::Result<PathBuf> {
    let app_dir = try_app_dir()?;
    Ok(app_dir.join("logs"))
}
