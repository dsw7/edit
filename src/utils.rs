use std::fs;
use std::path;

use anyhow::Context;

pub fn write_to_file(file: &path::Path, contents: &str) -> anyhow::Result<()> {
    fs::write(file, contents).context(format!("failed to write to file `{}`", &file.display()))
}
