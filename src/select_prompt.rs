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

pub fn select_prompt(user_prompt_from_cli: &Option<String>) -> Result<String, io::Error> {
    match user_prompt_from_cli {
        Some(prompt) => Ok(prompt.to_string()),
        None => match load_prompt_from_file() {
            Ok(file_prompt) => Ok(file_prompt),
            Err(_) => load_prompt_from_stdin(),
        },
    }
}
