use anyhow::Context;

use crate::configurations::Configs;
use crate::query_anthropic::{AnthropicParams, edit_code_block};

use super::resolve_programming_language::resolve_lang_from_extension;

pub fn run_process(configs: Configs) -> anyhow::Result<String> {
    if configs.instructions.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    let lang = resolve_lang_from_extension(&configs.filename)?;

    let results = edit_code_block(&AnthropicParams {
        code_to_edit: configs.code_to_edit,
        max_tokens: configs.max_tokens_edit_model,
        model: configs.code_edit_model,
        programming_language: lang,
        prompt: configs.instructions,
    })
    .context("editing process failed")?;

    let json_str =
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?;
    Ok(json_str)
}
