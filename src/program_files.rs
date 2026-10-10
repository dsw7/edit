use std::env;
use std::path::PathBuf;

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
