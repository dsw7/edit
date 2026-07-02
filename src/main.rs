mod query_openai;

use std::path::PathBuf;

use clap::Parser;

use query_openai::query_openai;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    if let Err(err) = query_openai() {
        eprintln!("Error: {}", err);
    }
}
