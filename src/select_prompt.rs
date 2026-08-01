use std::fs;
use std::io::{self, Write};

use anyhow::{self, Context};

fn load_prompt_from_file() -> anyhow::Result<String> {
    if fs::metadata("Inputfile").is_ok() {
        return fs::read_to_string("Inputfile").context("Failed to read the content of Inputfile");
    }

    Err(anyhow::anyhow!("Inputfile does not exist"))
}

fn load_prompt_from_stdin() -> anyhow::Result<String> {
    print!(">>> ");
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut prompt_from_stdin = String::new();
    io::stdin()
        .read_line(&mut prompt_from_stdin)
        .context("Failed to read line from stdin")?;

    Ok(prompt_from_stdin)
}

pub fn select_prompt(user_prompt_from_cli: &Option<String>) -> anyhow::Result<String> {
    let user_prompt = match user_prompt_from_cli {
        Some(prompt) => prompt.to_string(),
        None => match load_prompt_from_file() {
            Ok(file_prompt) => file_prompt,
            Err(_) => load_prompt_from_stdin()?,
        },
    };

    let user_prompt = user_prompt.trim().to_string();

    if user_prompt.is_empty() {
        anyhow::bail!("Cannot proceed. The prompt is empty")
    } else {
        Ok(user_prompt)
    }
}
