mod core;
mod params;
mod program_files;
mod query_openai;
mod utils;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use params::CliParameters;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,

    #[arg(
        short,
        long,
        default_value = "gpt-4o",
        help = "Specify provider specific LLM"
    )]
    model: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let params = CliParameters {
        input_file: cli.file_to_edit,
        model: cli.model,
    };

    match core::run_process(params) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
