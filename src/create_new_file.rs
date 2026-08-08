use crate::params::CliParameters;
use crate::query_openai;
use crate::utils;

pub fn create_new_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let results = query_openai::run_query(cli_params.model, user_prompt)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    println!("Created new file `{}`", &cli_params.input_file.display());

    Ok(results)
}
