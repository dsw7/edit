use crate::params::Parameters;
use crate::query_openai::{OpenAIResults, query_openai};

use std::fs;
use std::io::{self, Write};

fn load_prompt_from_stdin() -> Result<String, io::Error> {
    print!(">>> ");
    io::stdout().flush()?;

    let mut prompt_from_stdin = String::new();
    io::stdin().read_line(&mut prompt_from_stdin)?;

    let prompt_trimmed = prompt_from_stdin.trim().to_string();
    Ok(prompt_trimmed)
}

fn get_user_prompt(user_prompt: &Option<String>) -> Result<String, io::Error> {
    match user_prompt {
        Some(prompt) => Ok(prompt.to_string()),
        None => load_prompt_from_stdin(),
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
    let prompt = get_user_prompt(&params.prompt)?;

    let text_to_edit = fs::read_to_string(&params.input_file)?;
    let prompt_updated = update_user_prompt(prompt, text_to_edit);

    let results = query_openai(&prompt_updated, &params.model)?;
    fs::write(&params.input_file, &results.code)?;

    Ok(results)
}

fn operate_on_new_file(params: &Parameters) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    let prompt = get_user_prompt(&params.prompt)?;
    let results = query_openai(&prompt, &params.model)?;

    println!("Created new file: '{}'", &params.input_file.display());
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
