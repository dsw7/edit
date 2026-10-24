use std::collections::HashMap;
use std::path::Path;

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
        ("md", ("Markdown", "markdown")),
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

pub fn resolve_lang_from_extension(
    filename: &Path,
) -> anyhow::Result<(&'static str, &'static str)> {
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
