mod configs;
mod core;
mod params;
mod program_files;
mod query_openai;
mod utils;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use configs::load_configs;
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

    let configs = match load_configs() {
        Ok(configs) => configs,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let params = CliParameters {
        input_file: cli.file_to_edit,
        model: configs.openai.model,
    };

    match core::run_process(params) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
