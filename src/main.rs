#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod cli;
mod core;
mod run_init;
mod run_update;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands, UpdateConfigs};
use core::setup_configurations;
use run_init::initialize_program;
use run_update::update_code;

fn exec_update_subcommand(configs: UpdateConfigs) -> anyhow::Result<()> {
    let configs = setup_configurations(configs)?;
    let results = update_code(&configs)?;
    println!("{results}");
    Ok(())
}

fn execute_subcommand() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => initialize_program(),
        Commands::Update(configs) => exec_update_subcommand(configs),
    }
}

fn main() -> ExitCode {
    match execute_subcommand() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:?}");
            ExitCode::FAILURE
        }
    }
}
