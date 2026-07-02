use std::error::Error;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use reqwest::blocking::Client;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,
}

fn query_llm() -> Result<(), Box<dyn Error>> {
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let response = client.get("https://www.rust-lang.org").send()?;
    println!("Body text: {}", response.text()?);
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let _ = query_llm();
}
