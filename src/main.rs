use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use reqwest::blocking::Client;
use serde_json::json;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,
}

fn query_llm() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;

    let input = json!([
        { "role": "system", "content": "You are a helpful assistant." },
        { "role": "user", "content": "What is 3 + 5?" }
    ]);

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
    let response = client
        .post("https://api.openai.com/v1/responses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({ "model": "gpt-4o", "input": input }))
        .send()?;

    let response_text = response.text()?;
    println!("{}", response_text);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = query_llm() {
        eprintln!("Error: {}", err);
    }
}
