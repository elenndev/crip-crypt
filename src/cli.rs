use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encrypt {
        // Positional arguments
        folder_path: PathBuf,
        output: PathBuf,

        #[arg(short, long)]
        password: String,
    },

    Decrypt {
        // Positional arguments
        encrypted_file: PathBuf,
        output: PathBuf,

        #[arg(short, long)]
        password: String,
    },
}
