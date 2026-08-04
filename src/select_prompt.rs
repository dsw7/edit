use std::io::{self, Write};
use std::path::Path;

use crate::utils;

use anyhow::Context;

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

    let prompt = utils::read_file(&input_file)?;
    Ok(prompt)
}

pub fn load_prompt_from_file_or_stdin() -> anyhow::Result<String> {
    let input_file = Path::new("Inputfile");

    if input_file.exists() {
        load_prompt_from_file(&input_file)
    } else {
        load_prompt_from_stdin()
    }
}
