use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
pub struct ConfigsFromCli {
    #[arg(value_name = "FILE-TO-EDIT")]
    pub file_to_edit: PathBuf,

    #[arg(short, long, help = "Specify provider (openai, ...)")]
    pub provider: Option<String>,
}
