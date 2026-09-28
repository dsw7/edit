use std::fs;

use anyhow::Context;
use crossterm::style::Stylize;

use crate::configurations::Configs;
use crate::query_anthropic::{AnthropicParams, AnthropicResults, write_new_code};

pub fn create_new_file(configs: Configs, user_prompt: &str) -> anyhow::Result<AnthropicResults> {
    let results = write_new_code(
        user_prompt,
        &AnthropicParams {
            model: configs.code_edit_model,
            max_tokens: configs.max_tokens_edit_model,
        },
    )?;

    fs::write(&configs.input_file, &results.code).context(format!(
        "failed to write to file `{}`",
        &configs.input_file.display()
    ))?;

    print!("Created new file ");
    let input_file = format!("{}", &configs.input_file.display());
    println!("{}", input_file.blue());

    Ok(results)
}
