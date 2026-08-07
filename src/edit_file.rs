use anyhow::Context;

use crate::params::CliParameters;
use crate::query_openai;
use crate::select_prompt::load_prompt_from_file_or_stdin;
use crate::utils;

fn extract_user_prompt(cli_params: &CliParameters) -> anyhow::Result<String> {
    let prompt = match &cli_params.prompt {
        Some(prompt) => prompt.to_string(),
        None => load_prompt_from_file_or_stdin()?,
    };

    let prompt = prompt.trim().to_string();

    if prompt.is_empty() {
        anyhow::bail!("The prompt is empty")
    } else {
        Ok(prompt)
    }
}

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

fn operate_on_existing_file(
    cli_params: &CliParameters,
) -> anyhow::Result<query_openai::OpenAIResults> {
    let user_prompt = extract_user_prompt(cli_params)?;
    let text_to_edit = utils::read_file(&cli_params.input_file)?;

    let params = query_openai::OpenAIParams {
        model: cli_params.model.clone(),
        prompt: update_user_prompt(user_prompt, text_to_edit),
    };
    let results = query_openai::run_query(&params)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    Ok(results)
}

fn operate_on_new_file(cli_params: &CliParameters) -> anyhow::Result<query_openai::OpenAIResults> {
    let user_prompt = extract_user_prompt(cli_params)?;

    let params = query_openai::OpenAIParams {
        model: cli_params.model.clone(),
        prompt: user_prompt,
    };
    let results = query_openai::run_query(&params)?;

    utils::write_to_file(&cli_params.input_file, &results.code)?;
    println!("Created new file `{}`", &cli_params.input_file.display());

    Ok(results)
}

fn operate_on_file(cli_params: &CliParameters) -> anyhow::Result<query_openai::OpenAIResults> {
    if cli_params.input_file.exists() {
        operate_on_existing_file(cli_params)
    } else {
        operate_on_new_file(cli_params)
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
