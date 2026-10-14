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

use cli::{Cli, Commands, EditConfigs};
use configurations::setup_configurations;
use run_process::run_process;

fn exec_edit_subcommand(configs: EditConfigs) -> anyhow::Result<()> {
    let configs = setup_configurations(configs)?;
    let results = run_process(&configs)?;
    println!("{results}");
    Ok(())
}

fn execute_subcommand() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install {} => unimplemented!("Not ready"),
        Commands::Edit(configs) => exec_edit_subcommand(configs),
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
