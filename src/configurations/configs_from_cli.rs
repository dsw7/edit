use std::path::PathBuf;

use clap::Parser;

fn non_empty_argument(arg: &str) -> Result<String, String> {
    if arg.is_empty() {
        Err(String::from("the argument must not be empty"))
    } else {
        Ok(arg.to_string())
    }
}

#[derive(Parser, Debug)]
#[command(about = "Program for editing individual files using LLMs.", version)]
pub struct ConfigsFromCli {
    #[arg(value_name = "CODE-TO-EDIT", value_parser = non_empty_argument)]
    pub code_to_edit: String,

    #[arg(value_name = "INSTRUCTIONS", value_parser = non_empty_argument)]
    pub instructions: String,

    /// Specify filename (used to infer programming language)
    #[arg(short, long)]
    pub filename: PathBuf,
}
