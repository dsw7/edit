use anyhow::Context;

use super::create_new_file::create_new_file;
use super::edit_existing_file::edit_existing_file;
use crate::configurations::Configs;
use crate::query_anthropic::AnthropicResults;

fn operate_on_file(configs: Configs, user_prompt: &str) -> anyhow::Result<AnthropicResults> {
    if configs.input_file.exists() {
        edit_existing_file(configs, user_prompt)
    } else {
        create_new_file(configs, user_prompt)
    }
}

pub fn run_process(configs: Configs) -> anyhow::Result<()> {
    let user_prompt = String::from("test");

    if user_prompt.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    let results = operate_on_file(configs, &user_prompt).context("editing process failed")?;
    Ok(())
}
