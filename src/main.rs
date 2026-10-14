#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod cli;
mod core;
mod edit;

use std::process::ExitCode;

use clap::Parser;

use cli::{Cli, Commands, EditConfigs};
use core::setup_configurations;
use edit::edit_code;

fn exec_edit_subcommand(configs: EditConfigs) -> anyhow::Result<()> {
    let configs = setup_configurations(configs)?;
    let results = edit_code(&configs)?;
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
