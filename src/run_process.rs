use std::collections::HashMap;
use std::path::Path;

use anyhow::Context;
use serde::Serialize;

use crate::configurations::Configs;
use crate::query_anthropic::edit_code_block;

fn possible_languages() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("cc", "C++"),
        ("cpp", "C++"),
        ("c", "C"),
        ("cs", "C#"),
        ("css", "CSS"),
        ("html", "HTML"),
        ("js", "JavaScript"),
        ("json", "JSON"),
        ("java", "Java"),
        ("kt", "Kotlin"),
        ("pl", "Perl"),
        ("php", "PHP"),
        ("py", "Python"),
        ("rb", "Ruby"),
        ("rs", "Rust"),
        ("sh", "Shell"),
        ("sql", "SQL"),
        ("swift", "Swift"),
        ("ts", "TypeScript"),
        ("xml", "XML"),
        ("yaml", "YAML"),
        ("yml", "YAML"),
    ])
}

fn resolve_lang_from_extension(input_file: &Path) -> anyhow::Result<String> {
    let ext_os = input_file
        .extension()
        .ok_or_else(|| anyhow::anyhow!("could not get extension from file"))?;

    let extension = ext_os
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("extension is not valid UTF-8"))?;

    let lang_map = possible_languages();

    match lang_map.get(extension) {
        Some(lang) => Ok(lang.to_string()),
        None => anyhow::bail!(format!(
            "cannot resolve language from extension `{extension}`"
        )),
    }
}

#[derive(Serialize)]
struct Results {
    code: String,
    input_tokens: u32,
    lang_id: String,
    output_tokens: u32,
}

pub fn run_process(configs: &Configs) -> anyhow::Result<String> {
    if configs.prompt.is_empty() {
        anyhow::bail!("the user prompt is empty")
    }

    let language = resolve_lang_from_extension(&configs.filename)?;
    let raw_results = edit_code_block(configs, &language).context("editing process failed")?;

    let results = Results {
        code: raw_results.code,
        input_tokens: raw_results.input_tokens,
        output_tokens: raw_results.output_tokens,
        lang_id: language,
    };

    let json =
        serde_json::to_string_pretty(&results).context("failed to serialize outgoing results")?;
    Ok(json)
}
