use crate::params::Parameters;
use crate::query_openai::{OpenAIResults, query_openai};

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

fn read_file_to_string(file: &PathBuf) -> Result<String, io::Error> {
    let mut file_handle = File::open(file)?;

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;

    Ok(file_content)
}

fn update_user_prompt(user_prompt: &String, text_to_edit: String) -> String {
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
    let text_to_edit = read_file_to_string(&params.input_file)?;

    let results = query_openai(&params)?;
    fs::write(&params.input_file, &results.code)?;

    Ok(results)
}

fn operate_on_new_file(params: &Parameters) -> Result<OpenAIResults, Box<dyn std::error::Error>> {
    println!(
        "File '{}' does not exist. Will create new file",
        &params.input_file.display()
    );

    let results = query_openai(&params)?;
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
