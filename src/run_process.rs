use anyhow::Context;

use crate::configurations::Configs;
use crate::query_anthropic::edit_code_block;

pub fn run_process(configs: &Configs) -> anyhow::Result<String> {
    if configs.prompt.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    let results = edit_code_block(configs).context("editing process failed")?;

    let json_str =
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?;
    Ok(json_str)
}
