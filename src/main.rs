mod create_new_file;
mod edit_existing_file;
mod edit_file;
mod params;
mod query_openai;
mod select_prompt;
mod utils;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use edit_file::edit_file;
use params::CliParameters;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,

    #[arg(short, long, help = "Specify prompt via command line")]
    prompt: Option<String>,

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
        prompt: cli.prompt,
    };

    match edit_file(&params) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
