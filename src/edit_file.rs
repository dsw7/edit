use anyhow::Context;

use crate::create_new_file::create_new_file;
use crate::edit_existing_file::edit_existing_file;
use crate::params::CliParameters;
use crate::query_openai::OpenAIResults;

fn operate_on_file(cli_params: &CliParameters) -> anyhow::Result<OpenAIResults> {
    if cli_params.input_file.exists() {
        edit_existing_file(cli_params)
    } else {
        create_new_file(cli_params)
    }
}

pub fn edit_file(cli_params: &CliParameters) -> anyhow::Result<()> {
    let results = operate_on_file(cli_params).context("Editing process failed")?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!(
        "Description of what was done: {}",
        results.description_of_what_was_done
    );

    Ok(())
}
