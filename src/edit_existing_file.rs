use anyhow::Context;

use std::fs::OpenOptions;
use std::io::Write;

use crate::params::CliParameters;
use crate::query_openai;
use crate::utils;

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

const DELIM_EDIT_CODE: &str = "@@@\n";
const EDIT_START: &str = ">>>>>>>\n";
const EDIT_END: &str = "<<<<<<<\n";

pub fn edit_existing_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let mut file_content = utils::read_file(&cli_params.input_file)?;

    let start_idx = match file_content.find(DELIM_EDIT_CODE) {
        Some(idx) => idx,
        None => anyhow::bail!("Opening delimiter not found"),
    };

    let end_idx = match file_content[start_idx + DELIM_EDIT_CODE.len()..].find(DELIM_EDIT_CODE) {
        Some(idx) => start_idx + DELIM_EDIT_CODE.len() + idx + DELIM_EDIT_CODE.len(),
        None => anyhow::bail!("Closing delimiter not found"),
    };

    let inner_start = start_idx + DELIM_EDIT_CODE.len();
    let inner_end = end_idx - DELIM_EDIT_CODE.len();
    let inner_content = &mut file_content[inner_start..inner_end];

    let params = query_openai::OpenAIParams {
        model: cli_params.model,
        prompt: update_user_prompt(user_prompt, inner_content),
    };
    let results = query_openai::run_query(params).context("OpenAI query failed")?;

    let new_text = format!("{}{}{}", EDIT_START, results.code, EDIT_END);
    file_content.replace_range(start_idx..end_idx, &new_text);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cli_params.input_file)
        .context(format!(
            "Failed to open file `{}`",
            &cli_params.input_file.display()
        ))?;

    file.write_all(file_content.as_bytes()).context(format!(
        "Failed to write to file `{}`",
        &cli_params.input_file.display()
    ))?;
    Ok(results)
}
