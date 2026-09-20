use std::fs;

use anyhow::Context;
use crossterm::style::Stylize;

use crate::configurations::Configs;
use crate::query_anthropic::{AnthropicResults, write_new_code};

pub fn create_new_file(params: Configs, user_prompt: &str) -> anyhow::Result<AnthropicResults> {
    let results = write_new_code(
        params.max_tokens_edit_model,
        &params.code_edit_model,
        user_prompt,
    )?;

    fs::write(&params.input_file, &results.code).context(format!(
        "failed to write to file `{}`",
        &params.input_file.display()
    ))?;

    print!("Created new file ");
    let input_file = format!("{}", &params.input_file.display());
    println!("{}", input_file.blue());

    Ok(results)
}
