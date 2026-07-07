use std::fs;
use std::io::{self, Write};

fn load_prompt_from_stdin() -> Result<String, io::Error> {
    print!(">>> ");
    io::stdout().flush()?;

    let mut prompt_from_stdin = String::new();
    io::stdin().read_line(&mut prompt_from_stdin)?;

    let prompt = prompt_from_stdin.trim().to_string();
    Ok(prompt)
}

fn load_prompt_from_file() -> Result<String, io::Error> {
    if fs::metadata("Inputfile").is_ok() {
        let prompt_from_file = fs::read_to_string("Inputfile")?;
        return Ok(prompt_from_file.trim().to_string());
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Inputfile not found",
    ))
}

fn is_prompt_empty(user_prompt: Result<String, io::Error>) -> Result<String, io::Error> {
    match user_prompt {
        Ok(prompt) => {
            if prompt.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Prompt cannot be empty",
                ))
            } else {
                Ok(prompt)
            }
        }
        Err(error) => Err(error),
    }
}

pub fn select_prompt(user_prompt_from_cli: &Option<String>) -> Result<String, io::Error> {
    let user_prompt = match user_prompt_from_cli {
        Some(prompt) => Ok(prompt.to_string()),
        None => match load_prompt_from_file() {
            Ok(file_prompt) => Ok(file_prompt),
            Err(_) => load_prompt_from_stdin(),
        },
    };

    is_prompt_empty(user_prompt)
}
