#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod cli;
mod configurations;
mod program_files;
mod query_anthropic;
mod run_process;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands, RunConfigs};
use configurations::setup_configurations;
use run_process::run_process;

fn run(configs: RunConfigs) -> anyhow::Result<()> {
    let configs = setup_configurations(configs)?;
    let results = run_process(&configs)?;
    println!("{results}");
    Ok(())
}

fn run_subcommand() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {} => unimplemented!("Not ready"),
        Commands::Run(configs) => run(configs),
    }
}

fn main() -> ExitCode {
    match run_subcommand() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
