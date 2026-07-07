use std::io::{self, Write};

fn load_prompt_from_stdin() -> Result<String, io::Error> {
    print!(">>> ");
    io::stdout().flush()?;

    let mut prompt_from_stdin = String::new();
    io::stdin().read_line(&mut prompt_from_stdin)?;

    let prompt_trimmed = prompt_from_stdin.trim().to_string();
    Ok(prompt_trimmed)
}

pub fn select_prompt(user_prompt: &Option<String>) -> Result<String, io::Error> {
    match user_prompt {
        Some(prompt) => Ok(prompt.to_string()),
        None => load_prompt_from_stdin(),
    }
}
