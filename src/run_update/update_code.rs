use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use serde::Serialize;

use super::anthropic::query_messages_api;
use crate::core::Configs;

#[derive(Default, Serialize)]
struct Results {
    input_tokens: u32,
    lang_id: String,
    language: String,
    model: String,
    output_tokens: u32,
    updated_code: String,
}

fn possible_languages() -> HashMap<&'static str, (&'static str, &'static str)> {
    HashMap::from([
        ("cc", ("C++", "cpp")),
        ("cpp", ("C++", "cpp")),
        ("c", ("C", "c")),
        ("cs", ("C#", "cs")),
        ("css", ("CSS", "css")),
        ("html", ("HTML", "html")),
        ("js", ("JavaScript", "javascript")),
        ("json", ("JSON", "json")),
        ("java", ("Java", "java")),
        ("kt", ("Kotlin", "kotlin")),
        ("pl", ("Perl", "perl")),
        ("php", ("PHP", "php")),
        ("py", ("Python", "python")),
        ("rb", ("Ruby", "ruby")),
        ("rs", ("Rust", "rust")),
        ("sh", ("Shell", "sh")),
        ("sql", ("SQL", "sql")),
        ("swift", ("Swift", "swift")),
        ("ts", ("TypeScript", "typescript")),
        ("vim", ("Vimscript", "vim")),
        ("xml", ("XML", "xml")),
        ("yaml", ("YAML", "yaml")),
        ("yml", ("YAML", "yaml")),
    ])
}

fn resolve_lang_from_extension(filename: &Path) -> anyhow::Result<(&'static str, &'static str)> {
    let ext_os = filename
        .extension()
        .ok_or_else(|| anyhow::anyhow!("could not get extension from file"))?;

    let extension = ext_os
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("extension is not valid UTF-8"))?;

    let lang_map = possible_languages();

    match lang_map.get(extension) {
        Some(pair) => Ok(*pair),
        None => anyhow::bail!("cannot resolve language from extension `{extension}`"),
    }
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

pub fn update_code(configs: &Configs) -> anyhow::Result<String> {
    let results = if configs.echo {
        echo_payload_back_to_client(configs)?
    } else {
        process_code_using_anthropic(configs)?
    };

    let json = if configs.pretty {
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?
    } else {
        serde_json::to_string(&results).context("failed to serialize outgoing results")?
    };

    Ok(json)
}
