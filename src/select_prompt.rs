use std::fs;
use std::io::{self, Write};
use std::path::Path;

use anyhow::{self, Context};

fn load_prompt_from_stdin() -> anyhow::Result<String> {
    print!(">>> ");
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .context("Failed to read line from stdin")?;

    Ok(prompt)
}

fn load_prompt_from_file() -> anyhow::Result<String> {
    println!("Found Inputfile in current directory. Reading instructions from this file");

    let prompt =
        fs::read_to_string("Inputfile").context("Failed to read the content of Inputfile")?;

    Ok(prompt)
}

fn load_prompt_from_file_or_stdin() -> anyhow::Result<String> {
    if Path::new("Inputfile").exists() {
        load_prompt_from_file()
    } else {
        load_prompt_from_stdin()
    }
}

pub fn select_prompt(user_prompt_from_cli: &Option<String>) -> anyhow::Result<String> {
    let user_prompt = match user_prompt_from_cli {
        Some(prompt) => prompt.to_string(),
        None => load_prompt_from_file_or_stdin()?,
    };

    let user_prompt = user_prompt.trim().to_string();

    if user_prompt.is_empty() {
        anyhow::bail!("Cannot proceed. The prompt is empty")
    } else {
        Ok(user_prompt)
    }
}
