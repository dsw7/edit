mod configurations;
mod core;
mod program_files;
mod query_ollama;
mod query_openai;
mod utils;

use std::process::ExitCode;

use crossterm::style::Stylize;

use configurations::load_configs;

fn main() -> ExitCode {
    let configs = match load_configs() {
        Ok(configs) => configs,
        Err(error) => {
            eprintln!("{error:?}");
            return ExitCode::FAILURE;
        }
    };

    match core::run_process(configs) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let errmsg = format!("{error:?}");
            eprintln!("{}", errmsg.red());
            ExitCode::FAILURE
        }
    }
}
