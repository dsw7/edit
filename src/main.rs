mod configs;
mod core;
mod params;
mod program_files;
mod query_ollama;
mod query_openai;
mod utils;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use crossterm::style::Stylize;

use configs::load_configs;
use params::Parameters;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
struct Cli {
    #[arg(value_name = "FILE-TO-EDIT")]
    file_to_edit: PathBuf,

    #[arg(short, long, help = "Specify provider (openai, ...)")]
    provider: Option<String>,
}

fn load_params() -> anyhow::Result<Parameters> {
    let cli = Cli::parse();
    let configs = load_configs()?;

    let provider = match cli.provider {
        Some(provider) => provider,
        None => configs.provider,
    };

    let model = match provider.as_str() {
        "openai" => configs.openai.model,
        _ => anyhow::bail!(format!("invalid provider: `{provider}`")),
    };

    let params = Parameters {
        enable_prompt_validation: configs.enable_prompt_validation,
        input_file: cli.file_to_edit,
        model,
        provider,
    };

    Ok(params)
}

fn main() -> ExitCode {
    let params = match load_params() {
        Ok(params) => params,
        Err(error) => {
            eprintln!("{error:?}");
            return ExitCode::FAILURE;
        }
    };

    match core::run_process(params) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let errmsg = format!("{error:?}");
            eprintln!("{}", errmsg.red());
            ExitCode::FAILURE
        }
    }
}
