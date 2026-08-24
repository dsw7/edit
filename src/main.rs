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

    #[arg(short, long, help = "Specify provider (openai, ...)")]
    provider: Option<String>,
}

fn load_params() -> anyhow::Result<CliParameters> {
    let cli = Cli::parse();
    let configs = load_configs()?;

    let provider = match cli.provider {
        Some(provider) => provider,
        None => configs.provider,
    };

    let model = match provider.as_str() {
        "openai" => configs.openai.model,
        _ => anyhow::bail!(format!("Invalid provider: `{provider}`")),
    };

    let params = CliParameters {
        input_file: cli.file_to_edit,
        model,
    };

    Ok(params)
}

fn main() -> ExitCode {
    let params = match load_params() {
        Ok(params) => params,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    match core::run_process(params) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
