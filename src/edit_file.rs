use crate::params::Parameters;
use crate::query_openai::query_openai;

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

fn create_file_with_prompt(file: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    print!("File does not exist. Do you want to create it? [y/n]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim().eq_ignore_ascii_case("y") {
        File::create(file)?;
    } else {
        return Err(Box::from(
            "File does not exist and was not created. Cannot proceed!",
        ));
    }

    Ok(())
}

fn read_file_to_string(file: &PathBuf) -> Result<String, io::Error> {
    let mut file_handle = File::open(file)?;

    let mut file_content = String::new();
    file_handle.read_to_string(&mut file_content)?;

    Ok(file_content)
}

pub fn edit_file(
    file_to_edit: &PathBuf,
    params: &Parameters,
) -> Result<(), Box<dyn std::error::Error>> {
    if !file_to_edit.exists() {
        create_file_with_prompt(&file_to_edit)?;
    }

    let text_to_edit = read_file_to_string(&file_to_edit)?;

    let results = query_openai(&params)?;

    println!("Input tokens: {}", results.input_tokens);
    println!("Output tokens: {}", results.output_tokens);
    println!("Code: {}", results.code);
    println!(
        "Description of what was done: {}",
        results.description_of_what_was_done
    );

    fs::write(file_to_edit, results.code)?;

    Ok(())
}
