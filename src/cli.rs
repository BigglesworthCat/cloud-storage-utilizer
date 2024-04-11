use crate::APPLICATION_NAME;
use clap::{Error, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Command {
    Download {
        from_path: PathBuf,
        to_path: PathBuf,
    },
    Upload {
        from_path: PathBuf,
        to_path: PathBuf,
    },
    Delete {
        path: PathBuf,
    },
    List,
    Clear,
}

#[derive(Parser, Debug)]
#[command(name = "cloud-storage-utilizer", version = "0.0.1", about = "Performs specified request to cloud storage", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn parse_str(input: &str) -> Result<Cli, Error> {
        let command = APPLICATION_NAME.to_string() + " " + input.trim();
        Cli::try_parse_from(command.split(' '))
    }
}
