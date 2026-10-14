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

use cli::{Cli, Commands};
use run_init::run_init;
use run_update::run_update;

fn execute_subcommand() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => run_init(),
        Commands::Update(configs) => run_update(configs),
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
