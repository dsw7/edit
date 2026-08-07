use std::io::{self, Write};
use std::path::Path;

use anyhow::Context;

use crate::utils;
use crate::params::CliParameters;


fn load_prompt_from_stdin() -> anyhow::Result<String> {
    print!(">>> ");
    io::stdout().flush().context("Failed to flush stdout")?;

    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .context("Failed to read line from stdin")?;

    Ok(prompt)
}

fn load_prompt_from_file(input_file: &Path) -> anyhow::Result<String> {
    println!("Found Inputfile in current directory. Reading instructions from this file");

    let prompt = utils::read_file(input_file)?;
    Ok(prompt)
}

fn load_prompt_from_file_or_stdin() -> anyhow::Result<String> {
    let input_file = Path::new("Inputfile");

    if input_file.exists() {
        load_prompt_from_file(input_file)
    } else {
        load_prompt_from_stdin()
    }
}

pub fn select_prompt(cli_params: &CliParameters) -> anyhow::Result<String> {
    let prompt = match &cli_params.prompt {
        Some(prompt) => prompt.to_string(),
        None => load_prompt_from_file_or_stdin()?,
    };

    let prompt = prompt.trim().to_string();

    if prompt.is_empty() {
        anyhow::bail!("The prompt is empty")
    } else {
        Ok(prompt)
    }
}
