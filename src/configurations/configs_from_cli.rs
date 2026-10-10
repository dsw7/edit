use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
pub struct ConfigsFromCli {
    #[arg(value_name = "CODE-TO-EDIT")]
    pub code_to_edit: String,

    #[arg(value_name = "INSTRUCTIONS")]
    pub instructions: String,

    /// specify filename (used to infer programming language)
    #[arg(short, long)]
    pub filename: PathBuf,
}
