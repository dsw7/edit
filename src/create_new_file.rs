use crate::params::CliParameters;
use crate::query_openai;
use crate::utils;

pub fn create_new_file(
    cli_params: CliParameters,
    user_prompt: String,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let params = query_openai::OpenAIParams {
        model: cli_params.model.clone(),
        prompt: user_prompt.clone(),
    };

    let results = query_openai::run_query(&params)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    println!("Created new file `{}`", &cli_params.input_file.display());

    Ok(results)
}
