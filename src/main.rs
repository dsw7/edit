mod params;
mod query_openai;
mod response_openai;

use std::path::PathBuf;
use std::process;

use clap::Parser;

use params::Parameters;
use query_openai::query_openai;

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
        prompt: cli.prompt,
        model: cli.model,
        client_timeout: 10,
    };

    if let Err(err) = query_openai(&params) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
