use anyhow::Context;

use crate::params::Parameters;
use crate::query_openai::{OpenAIResults, run_query};
use crate::select_prompt::load_prompt_from_file_or_stdin;
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

fn extract_user_prompt(params: &Parameters) -> anyhow::Result<String> {
    let prompt = match &params.prompt {
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

fn operate_on_existing_file(params: &Parameters) -> anyhow::Result<OpenAIResults> {
    let user_prompt = extract_user_prompt(params)?;

    let text_to_edit = utils::read_file(&params.input_file)?;
    let prompt_updated = update_user_prompt(user_prompt, text_to_edit);
    let results = run_query(&prompt_updated, &params.model)?;

    utils::write_to_file(&params.input_file, &results.code)?;
    Ok(results)
}

fn operate_on_new_file(params: &Parameters) -> anyhow::Result<OpenAIResults> {
    let user_prompt = extract_user_prompt(params)?;
    let results = run_query(&user_prompt, &params.model)?;

    utils::write_to_file(&params.input_file, &results.code)?;
    println!("Created new file `{}`", &params.input_file.display());

    Ok(results)
}

fn operate_on_file(params: &Parameters) -> anyhow::Result<OpenAIResults> {
    if params.input_file.exists() {
        operate_on_existing_file(params)
    } else {
        operate_on_new_file(params)
    }
}

pub fn edit_file(params: &Parameters) -> anyhow::Result<()> {
    let results = operate_on_file(params).context("Editing process failed")?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!(
        "Description of what was done: {}",
        results.description_of_what_was_done
    );

    Ok(())
}
