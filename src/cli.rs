use crate::APPLICATION_NAME;
use clap::{Error, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Debug, Clone)]
pub enum Workspace {
    Local,
    Cloud,
}

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
    List {
        workspace: Workspace,
    },
}

#[derive(Parser, Debug)]
#[command(name = "cloud-storage-utilizer", version = "0.0.1", about = "Performs specified request to cloud storage", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn parse_str(input: &String) -> Result<Cli, Error> {
        let mut command = APPLICATION_NAME.to_string();
        command.push(' ');
        command.push_str(input.to_string().trim());

        Cli::try_parse_from(command.split(' '))
    }
}
