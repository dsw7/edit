use std::fs;

use crate::core::{self, config_file};

fn create_app_dir() -> anyhow::Result<()> {
    let app_dir = core::try_app_dir()?;

    if app_dir.exists() {
        println!("program directory already exists");
    } else {
        fs::create_dir_all(&app_dir)?;
        println!("created application directory at: {}", app_dir.display());
    }

    Ok(())
}

fn create_config_file() -> anyhow::Result<()> {
    let config_file = core::try_config_file()?;

    if config_file.exists() {
        println!("program configuration file already exists")
    } else {
        println!("wrote new configuration file: {}", config_file.display());
        config_file::write_configs_to_file(&config_file)?;
    }

    Ok(())
}

pub fn run_init() -> anyhow::Result<()> {
    create_app_dir()?;
    create_config_file()?;
    Ok(())
}
