use crate::params::Parameters;
use crate::query_openai;
use crate::utils;

use crossterm::style::Stylize;

pub fn create_new_file(
    cli_params: Parameters,
    user_prompt: &str,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let results = query_openai::write_new_code(&cli_params.model, user_prompt)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;

    print!("Created new file ");
    let input_file = format!("{}", &cli_params.input_file.display());
    println!("{}", input_file.blue());

    Ok(results)
}
