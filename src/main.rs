mod edit_file;
mod params;
mod query_openai;
mod utils;

use std::path::PathBuf;
use std::process;

use clap::Parser;

use edit_file::edit_file;
use params::Parameters;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,

    #[arg(short, long, help = "Specify prompt via command line")]
    prompt: String,

    #[arg(
        short,
        long,
        default_value = "gpt-4o",
        help = "Specify provider specific LLM"
    )]
    model: String,
}

fn main() {
    let cli = Cli::parse();

    let params = Parameters {
        input_file: cli.file_to_edit,
        model: cli.model,
        prompt: cli.prompt,
    };

    if let Err(error) = edit_file(&params) {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
}
