use crate::params::CliParameters;
use crate::query_openai;
use crate::select_prompt::select_prompt;
use crate::utils;

pub fn create_new_file(cli_params: &CliParameters) -> anyhow::Result<query_openai::OpenAIResults> {
    let user_prompt = select_prompt(cli_params)?;

    let params = query_openai::OpenAIParams {
        model: cli_params.model.clone(),
        prompt: user_prompt,
    };
    let results = query_openai::run_query(&params)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    println!("Created new file `{}`", &cli_params.input_file.display());

    Ok(results)
}
