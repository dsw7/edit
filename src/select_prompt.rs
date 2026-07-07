use std::fs;
use std::io::{self, Write};

fn load_prompt_from_stdin() -> Result<String, io::Error> {
    print!(">>> ");
    io::stdout().flush()?;

    let mut prompt_from_stdin = String::new();
    io::stdin().read_line(&mut prompt_from_stdin)?;

    Ok(prompt_from_stdin)
}

fn load_prompt_from_file() -> Option<String> {
    if fs::metadata("Inputfile").is_ok() {
        if let Ok(prompt_from_file) = fs::read_to_string("Inputfile") {
            return Some(prompt_from_file);
        }
    }

    None
}

fn is_prompt_empty(user_prompt: Result<String, io::Error>) -> Result<String, io::Error> {
    match user_prompt {
        Ok(prompt) => {
            let prompt_trimmed = prompt.trim().to_string();

            if prompt_trimmed.is_empty() {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Prompt cannot be empty",
                ))
            } else {
                Ok(prompt_trimmed)
            }
        }
        Err(error) => Err(error),
    }
}

pub fn select_prompt(user_prompt_from_cli: &Option<String>) -> Result<String, io::Error> {
    let user_prompt = match user_prompt_from_cli {
        Some(prompt) => Ok(prompt.to_string()),
        None => match load_prompt_from_file() {
            Some(file_prompt) => Ok(file_prompt),
            None => load_prompt_from_stdin(),
        },
    };

    is_prompt_empty(user_prompt)
}
