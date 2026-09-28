use std::fs;

use anyhow::Context;
use crossterm::style::Stylize;

use crate::configurations::Configs;
use crate::query_anthropic::{AnthropicParams, AnthropicResults, write_new_code};

use super::resolve_programming_language::resolve_lang_from_extension;

pub fn create_new_file(configs: Configs, user_prompt: &str) -> anyhow::Result<AnthropicResults> {
    let lang = resolve_lang_from_extension(&configs.input_file)?;
    let results = write_new_code(
        user_prompt,
        &AnthropicParams {
            max_tokens: configs.max_tokens_edit_model,
            model: configs.code_edit_model,
            programming_language: lang,
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
