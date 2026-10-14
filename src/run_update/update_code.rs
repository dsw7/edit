use anyhow::Context;
use serde::Serialize;

use super::anthropic::query_messages_api;
use super::configs::Configs;
use super::resolve_language::resolve_lang_from_extension;
use crate::cli::UpdateConfigs;

#[derive(Default, Serialize)]
struct Results {
    input_tokens: u32,
    lang_id: String,
    language: String,
    model: String,
    output_tokens: u32,
    updated_code: String,
}

fn echo_payload_back_to_client(configs: &Configs) -> anyhow::Result<Results> {
    let (language, vim_syntax_lang_id) = resolve_lang_from_extension(&configs.filename)?;

    Ok(Results {
        lang_id: vim_syntax_lang_id.to_owned(),
        language: language.to_owned(),
        model: String::from("None"),
        updated_code: configs.code_to_edit.to_owned(),
        ..Default::default()
    })
}

fn process_code_using_anthropic(configs: &Configs) -> anyhow::Result<Results> {
    let (language, vim_syntax_lang_id) = resolve_lang_from_extension(&configs.filename)?;
    let response = query_messages_api(configs, language).context("reasoning process failed")?;

    Ok(Results {
        input_tokens: response.input_tokens,
        lang_id: vim_syntax_lang_id.to_owned(),
        language: language.to_owned(),
        model: configs.model.to_owned(),
        output_tokens: response.output_tokens,
        updated_code: response.code,
    })
}

pub fn update_code(configs_from_cli: UpdateConfigs) -> anyhow::Result<()> {
    let configs = Configs::from(configs_from_cli)
        .try_load_anthropic_api_key()?
        .try_load_configs_from_file()?;

    let results = if configs.echo {
        echo_payload_back_to_client(&configs)?
    } else {
        process_code_using_anthropic(&configs)?
    };

    let json = if configs.pretty {
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?
    } else {
        serde_json::to_string(&results).context("failed to serialize outgoing results")?
    };

    println!("{json}");
    Ok(())
}
