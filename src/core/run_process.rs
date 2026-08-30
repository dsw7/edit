use std::io::{self, Write};
use std::path::Path;

use anyhow::Context;
use crossterm::style::Stylize;
use crossterm::terminal;

use super::create_new_file::create_new_file;
use super::edit_existing_file::edit_existing_file;
use crate::params::Parameters;
use crate::query_ollama::is_valid_prompt;
use crate::query_openai::OpenAIResults;
use crate::utils;

fn get_term_width() -> usize {
    match terminal::size() {
        Ok(dimensions) => {
            let (width, _) = dimensions;
            usize::from(width)
        }
        Err(_) => 25,
    }
}

macro_rules! separator {
    ($width:expr) => {{
        let separator = "─".repeat($width);
        println!("{}", separator);
    }};
}

fn print_provider_info(params: &Parameters) {
    let editor = format!("{}:{}", &params.provider, &params.code_edit_model);
    println!("● Using {} for code editing", editor.green());

    let validator = format!("ollama:{}", &params.ollama_validation_model);
    println!("● Using {} for prompt validation", validator.green());

    println!();
    let q = "q";
    let quit = "quit";
    println!("Type {} | {} to quit", q.dark_grey(), quit.dark_grey());
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

fn should_exit_program(user_prompt: &str) -> bool {
    matches!(user_prompt, "quit" | "q")
}

fn prompt_is_invalid(params: &Parameters, user_prompt: &str) -> anyhow::Result<bool> {
    if params.disable_prompt_validation {
        return Ok(true);
    }

    let result_validation =
        is_valid_prompt(params, user_prompt).context("prompt validation process failed")?;

    if result_validation.valid_instructions {
        return Ok(false);
    }

    let errmsg = "Instructions failed validation";
    println!("! {}", errmsg.red());
    println!("! {}", result_validation.reasoning.dark_grey());

    let msg_usage = format!(
        "Validation took {} s | Input tokens: {} | Output tokens: {}",
        result_validation.total_duration,
        result_validation.input_tokens,
        result_validation.output_tokens
    );

    println!("! {}", msg_usage.dark_grey());
    Ok(true)
}

fn operate_on_file(params: Parameters, user_prompt: &str) -> anyhow::Result<OpenAIResults> {
    if params.input_file.exists() {
        edit_existing_file(params, user_prompt)
    } else {
        create_new_file(params, user_prompt)
    }
}

fn print_query_info(results: OpenAIResults) {
    println!();
    println!("● {}", results.description_of_what_was_done.dark_grey());

    let input_tokens = format!("{}", results.input_tokens);
    println!("● Input tokens: {}", input_tokens.green());

    let output_tokens = format!("{}", results.output_tokens);
    println!("● Output tokens: {}", output_tokens.green());
}

pub fn run_process(params: Parameters) -> anyhow::Result<()> {
    print_provider_info(&params);

    let term_width = get_term_width();
    separator!(term_width);

    let user_prompt = load_prompt_from_file_or_stdin()?;
    separator!(term_width);

    if should_exit_program(&user_prompt) {
        return Ok(());
    }

    if prompt_is_invalid(&params, &user_prompt)? {
        return Ok(());
    }

    let results = operate_on_file(params, &user_prompt).context("editing process failed")?;

    print_query_info(results);
    separator!(term_width);
    Ok(())
}
