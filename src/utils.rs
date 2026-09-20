use std::fs;
use std::path;

use anyhow::Context;

pub fn read_file(file: &path::Path) -> anyhow::Result<String> {
    fs::read_to_string(file).context(format!("failed to read file `{}`", &file.display()))
}

pub fn write_to_file(file: &path::Path, contents: &str) -> anyhow::Result<()> {
    fs::write(file, contents).context(format!("failed to write to file `{}`", &file.display()))
}
