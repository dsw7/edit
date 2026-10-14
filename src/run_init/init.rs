use std::fs;
use std::path::PathBuf;

use crate::core::{get_app_dir, write_configs_to_file};

fn create_app_dir() -> anyhow::Result<PathBuf> {
    let app_dir = get_app_dir()?;

    if !app_dir.exists() {
        fs::create_dir_all(&app_dir)?;
        println!("Created application directory at: {}", app_dir.display());
    }

    Ok(app_dir)
}

pub fn initialize_program() -> anyhow::Result<()> {
    create_app_dir()?;
    write_configs_to_file()?;
    Ok(())
}
