use crate::params::Parameters;
use crate::query_openai::{OpenAIResults, query_openai};

use std::fs;

fn get_user_prompt(user_prompt: &Option<String>) -> String {
    match user_prompt {
        Some(prompt) => prompt.to_string(),
        None => "Running a test".to_string(),
    }
}

fn update_user_prompt(user_prompt: String, text_to_edit: String) -> String {
    format!(
        "Take the instructions:
```plaintext
{}
```
And apply them to the code:
```
{}
```",
        user_prompt, text_to_edit
    )
}

fn operate_on_existing_file(
    params: &Parameters,
) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    let prompt = get_user_prompt(&params.prompt);

    let text_to_edit = fs::read_to_string(&params.input_file)?;
    let prompt_updated = update_user_prompt(prompt, text_to_edit);

    let results = query_openai(&prompt_updated, &params.model)?;
    fs::write(&params.input_file, &results.code)?;

    Ok(results)
}

fn operate_on_new_file(params: &Parameters) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    let prompt = get_user_prompt(&params.prompt);
    let results = query_openai(&prompt, &params.model)?;

    println!(
        "File '{}' does not exist. Will create new file",
        &params.input_file.display()
    );
    fs::write(&params.input_file, &results.code)?;

    Ok(results)
}

fn operate_on_file(params: &Parameters) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    if params.input_file.exists() {
        operate_on_existing_file(params)
    } else {
        operate_on_new_file(params)
    }
}

pub fn edit_file(params: &Parameters) -> Result<(), Box<dyn std::error::Error>> {
    let results = operate_on_file(params)?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!(
        "Description of what was done: {}",
        results.description_of_what_was_done
    );

    Ok(())
}
