use crate::APPLICATION_NAME;
use clap::{Error, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Download file from cloud storage to local machine
    Download {
        from_path: PathBuf,
        to_path: PathBuf,
    },
    /// Upload file from local machine to cloud storage
    Upload {
        from_path: PathBuf,
        to_path: PathBuf,
    },
    /// Delete file on cloud storage
    Delete {
        path: PathBuf,
    },
    /// List files on local machine and cloud storage
    List,
    /// Clear logs
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
