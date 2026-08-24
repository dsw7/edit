use std::io::{self, Write};
use std::path::Path;

use anyhow::Context;
use crossterm::style::{Stylize, style};

use super::create_new_file::create_new_file;
use super::edit_existing_file::edit_existing_file;
use crate::params::Parameters;
use crate::query_openai::OpenAIResults;
use crate::utils;

fn print_provider_info(params: &Parameters) {
    let provider = style(&params.provider).green();
    print!("● Using provider ");
    println!("{provider}");

    let model = style(&params.model).green();
    print!("● Using model ");
    println!("{model}");
}

fn load_prompt_from_stdin() -> anyhow::Result<String> {
    print!(">>> ");
    io::stdout().flush().context("failed to flush stdout")?;

    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .context("failed to read line from stdin")?;

    Ok(prompt)
}

fn load_prompt_from_file(input_file: &Path) -> anyhow::Result<String> {
    println!(">>> Found Inputfile in current directory. Reading instructions from this file");

    let prompt = utils::read_file(input_file)?;
    Ok(prompt)
}

fn load_prompt_from_file_or_stdin() -> anyhow::Result<String> {
    let input_file = Path::new("Inputfile");

    let user_prompt = if input_file.exists() {
        load_prompt_from_file(input_file)?
    } else {
        load_prompt_from_stdin()?
    };

    let user_prompt = user_prompt.trim().to_string();

    if user_prompt.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    Ok(user_prompt)
}

fn operate_on_file(params: Parameters, user_prompt: String) -> anyhow::Result<OpenAIResults> {
    if params.input_file.exists() {
        edit_existing_file(params, &user_prompt)
    } else {
        create_new_file(params, &user_prompt)
    }
}

fn print_query_info(results: OpenAIResults) {
    println!();
    print!("● ");
    println!("{}", results.description_of_what_was_done.dark_grey());

    print!("● Input tokens: ");
    let input_tokens = format!("{}", results.input_tokens);
    println!("{}", input_tokens.green());

    print!("● Output tokens: ");
    let output_tokens = format!("{}", results.output_tokens);
    println!("{}", output_tokens.green());
}

pub fn run_process(params: Parameters) -> anyhow::Result<()> {
    print_provider_info(&params);

    utils::print_sep()?;
    let user_prompt = load_prompt_from_file_or_stdin()?;
    utils::print_sep()?;

    let results = operate_on_file(params, user_prompt).context("editing process failed")?;

    print_query_info(results);
    utils::print_sep()?;
    Ok(())
}
