use std::fs;

use crate::core;

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
        core::try_write_configs_to_file(&config_file)?;
    }

    Ok(())
}

fn create_readme_file() -> anyhow::Result<()> {
    let app_dir = core::try_app_dir()?;
    let readme_file = app_dir.join("README.txt");

    if readme_file.exists() {
        println!("README.txt already exists")
    } else {
        let contents = "* `edit`\n* See https://github.com/dsw7/edit for more information\n";
        fs::write(&readme_file, contents)?;
        println!("wrote new README file: {}", readme_file.display());
    }

    Ok(())
}

pub fn run_init() -> anyhow::Result<()> {
    create_app_dir()?;
    create_config_file()?;
    create_readme_file()?;
    Ok(())
}
