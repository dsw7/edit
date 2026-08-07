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
