mod configurations;
mod program_files;
mod query_anthropic;
mod run_process;

use std::process::ExitCode;

use configurations::setup_configurations;
use run_process::run_process;

fn main() -> ExitCode {
    let configs = match setup_configurations() {
        Ok(configs) => configs,
        Err(error) => {
            eprintln!("{error:?}");
            return ExitCode::FAILURE;
        }
    };

    match run_process(configs) {
        Ok(results) => {
            println!("{results}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
