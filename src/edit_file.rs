use std::io::{self, Write};
use std::path::Path;

use anyhow::Context;

use crate::create_new_file::create_new_file;
use crate::edit_existing_file::edit_existing_file;
use crate::params::CliParameters;
use crate::query_openai::OpenAIResults;
use crate::utils;

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

fn operate_on_file(cli_params: CliParameters) -> anyhow::Result<OpenAIResults> {
    let user_prompt = load_prompt_from_file_or_stdin()?;
    let user_prompt = user_prompt.trim().to_string();

    if user_prompt.is_empty() {
        anyhow::bail!("The user prompt is empty")
    }

    if cli_params.input_file.exists() {
        edit_existing_file(cli_params, &user_prompt)
    } else {
        create_new_file(cli_params, &user_prompt)
    }
}

pub fn edit_file(cli_params: CliParameters) -> anyhow::Result<()> {
    let results = operate_on_file(cli_params).context("Editing process failed")?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!(
        "Description of what was done: {}",
        results.description_of_what_was_done
    );

    Ok(())
}
