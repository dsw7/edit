use anyhow::Context;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

use crate::params::CliParameters;
use crate::query_openai;
use crate::utils;

const DELIM_EDIT_CODE: &str = "@@@\n";
const EDIT_START: &str = ">>>>>>>\n";
const EDIT_END: &str = "<<<<<<<\n";

fn get_delim_indices(file_content: &str) -> anyhow::Result<(usize, usize)> {
    let start_idx = match file_content.find(DELIM_EDIT_CODE) {
        Some(idx) => idx,
        None => anyhow::bail!("Opening delimiter not found"),
    };

    let end_idx = match file_content[start_idx + DELIM_EDIT_CODE.len()..].find(DELIM_EDIT_CODE) {
        Some(idx) => start_idx + DELIM_EDIT_CODE.len() + idx + DELIM_EDIT_CODE.len(),
        None => anyhow::bail!("Closing delimiter not found"),
    };

    Ok((start_idx, end_idx))
}

fn get_delimited_block(
    file_content: &str,
    start_idx: usize,
    end_idx: usize,
) -> anyhow::Result<&str> {
    let inner_start = start_idx + DELIM_EDIT_CODE.len();
    let inner_end = end_idx - DELIM_EDIT_CODE.len();
    let inner_content = &file_content[inner_start..inner_end];

    if inner_content.trim().is_empty() {
        anyhow::bail!("Delimited content is empty")
    } else {
        Ok(inner_content)
    }
}

fn update_user_prompt(user_prompt: String, text_to_edit: &str) -> String {
    format!(
        "Take the instructions:
```plaintext
{user_prompt}
```
And apply them to the code:
```
{text_to_edit}
```"
    )
}

fn overwrite_file(filename: &PathBuf, content: String) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename)
        .context(format!("Failed to open file `{}`", &filename.display()))?;

    file.write_all(content.as_bytes())
        .context(format!("Failed to write to file `{}`", &filename.display()))?;

    Ok(())
}

pub fn edit_existing_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let mut file_content = utils::read_file(&cli_params.input_file)?;

    let (start_idx, end_idx) = get_delim_indices(&file_content)?;
    let inner_content = get_delimited_block(&file_content, start_idx, end_idx)?;

    let params = query_openai::OpenAIParams {
        model: cli_params.model,
        prompt: update_user_prompt(user_prompt, inner_content),
    };
    let results = query_openai::run_query(params).context("Failed to edit text using OpenAI")?;

    let new_text = format!("{}{}{}", EDIT_START, results.code, EDIT_END);
    file_content.replace_range(start_idx..end_idx, &new_text);

    overwrite_file(&cli_params.input_file, file_content)?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::{get_delim_indices, get_delimited_block};

    #[test]
    fn test_missing_start_delim() {
        let file_contents = "def foo(): pass\n";

        let result = get_delim_indices(file_contents);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Opening delimiter not found");
    }

    #[test]
    fn test_missing_end_delim() {
        let file_contents = "@@@\nprint('Hello world')\n";

        let result = get_delim_indices(file_contents);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Closing delimiter not found");
    }

    #[test]
    fn test_valid_start_end_delim() {
        let file_contents = "@@@\ndef foo(): pass\n@@@\n\nfoo()";

        let (start_idx, end_idx) = get_delim_indices(file_contents).unwrap();
        assert_eq!(start_idx, 0);
        assert_eq!(end_idx, 24);
    }

    #[test]
    fn test_get_delimited_block() {
        let file_contents = "@@@\ndef foo(): pass\n@@@\n\nfoo()";
        let block = get_delimited_block(file_contents, 0, 24).unwrap();
        assert_eq!(block, "def foo(): pass\n");
    }

    #[test]
    fn test_handle_empty_delimited_block() {
        let file_contents = "@@@\n@@@\n\nfoo()";
        let block = get_delimited_block(file_contents, 0, 8);

        assert!(block.is_err());
        assert_eq!(block.unwrap_err().to_string(), "Delimited content is empty");
    }

    #[test]
    fn test_handle_empty_delimited_block_2() {
        let file_contents = "@@@\n  \n@@@\n\nfoo()";
        let block = get_delimited_block(file_contents, 0, 11);

        assert!(block.is_err());
        assert_eq!(block.unwrap_err().to_string(), "Delimited content is empty");
    }
}
