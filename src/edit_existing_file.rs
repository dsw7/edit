use std::fs::OpenOptions;
use std::io::Write;

use crate::params::CliParameters;
use crate::query_openai;
use crate::utils;

fn update_user_prompt(user_prompt: String, text_to_edit: String) -> String {
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

/*
pub fn edit_existing_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let text_to_edit = utils::read_file(&cli_params.input_file)?;

    let params = query_openai::OpenAIParams {
        model: cli_params.model,
        prompt: update_user_prompt(user_prompt, text_to_edit),
    };
    let results = query_openai::run_query(params)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    Ok(results)
}
*/

const DELIM_EDIT_CODE: &str = "@@@\n";
const EDIT_START: &str = ">>>>>>>\n";
const EDIT_END: &str = "<<<<<<<\n";

pub fn edit_existing_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let mut file_content = utils::read_file(&cli_params.input_file)?;

    // Find the opening delimiter
    if let Some(start_idx) = file_content.find(DELIM_EDIT_CODE) {
        // Find the closing delimiter
        if let Some(end_idx) =
            file_content[start_idx + DELIM_EDIT_CODE.len()..].find(DELIM_EDIT_CODE)
        {
            let end_idx = start_idx + DELIM_EDIT_CODE.len() + end_idx + DELIM_EDIT_CODE.len();

            // Extract the opening delimiter, content between, and closing delimiter
            let inner_start = start_idx + DELIM_EDIT_CODE.len();
            let inner_end = end_idx - DELIM_EDIT_CODE.len();
            let inner_content = &mut file_content[inner_start..inner_end];

            // Process the lines between delimiters (edit them)
            // For demonstration, converting text to uppercase
            let processed_lines = inner_content.to_uppercase();

            // Reinsert the edited lines
            let new_text = format!("{}{}{}", EDIT_START, processed_lines, EDIT_END);
            file_content.replace_range(start_idx..end_idx, &new_text);

            // Write the updated content back to the file
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&cli_params.input_file)?;
            file.write_all(file_content.as_bytes())?;
        } else {
            anyhow::bail!("Closing delimiter not found");
        }
    }

    Ok(())
}
