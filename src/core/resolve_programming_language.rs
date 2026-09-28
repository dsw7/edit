use std::collections::HashMap;
use std::path::Path;

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

pub fn resolve_lang_from_extension(input_file: &Path) -> anyhow::Result<String> {
    if !input_file.is_file() {
        anyhow::bail!("provided path is not a file");
    }

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
