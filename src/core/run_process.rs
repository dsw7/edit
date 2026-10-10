use anyhow::Context;

use crate::configurations::Configs;
use crate::query_anthropic::{AnthropicParams, write_new_code};

use super::resolve_programming_language::resolve_lang_from_extension;

pub fn run_process(configs: &Configs) -> anyhow::Result<()> {
    if configs.instructions.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    let lang = resolve_lang_from_extension(&configs.filename)?;

    let results = write_new_code(
        &configs.instructions,
        &AnthropicParams {
            max_tokens: configs.max_tokens_edit_model,
            model: configs.code_edit_model,
            programming_language: lang,
        },
    )
    .context("editing process failed")?;

    Ok(())
}
