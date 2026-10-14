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

fn write_default_config_file() -> anyhow::Result<()> {
    let configs = ConfigsFromFile {
        anthropic: Anthropic {
            model: String::from("claude-haiku-4-5"),
            max_tokens: 4096,
        },
    };

    write_configs_to_file(configs)?;
    Ok(())
}

pub fn initialize_program() -> anyhow::Result<()> {
    create_app_dir()?;
    write_default_config_file()?;
    Ok(())
}
