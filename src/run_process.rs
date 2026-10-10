use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use serde::Serialize;

use crate::configurations::Configs;
use crate::query_anthropic::query_messages_api;

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
        ("xml", ("XML", "xml")),
        ("yaml", ("YAML", "yaml")),
        ("yml", ("YAML", "yaml")),
    ])
}

fn resolve_lang_from_extension(input_file: &Path) -> anyhow::Result<(&'static str, &'static str)> {
    let ext_os = input_file
        .extension()
        .ok_or_else(|| anyhow::anyhow!("could not get extension from file"))?;

    let extension = ext_os
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("extension is not valid UTF-8"))?;

    let lang_map = possible_languages();

    match lang_map.get(extension) {
        Some(lang) => Ok(*lang),
        None => anyhow::bail!("cannot resolve language from extension `{extension}`"),
    }
}

#[derive(Serialize)]
struct Results {
    input_tokens: u32,
    lang_id: String,
    output_tokens: u32,
    updated_code: String,
}

pub fn run_process(configs: &Configs) -> anyhow::Result<String> {
    let (language, vim_syntax_lang_id) = resolve_lang_from_extension(&configs.filename)?;
    let response = query_messages_api(configs, language).context("editing process failed")?;

    let results = Results {
        input_tokens: response.input_tokens,
        lang_id: vim_syntax_lang_id.to_string(),
        output_tokens: response.output_tokens,
        updated_code: response.code,
    };

    let json =
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?;
    Ok(json)
}
