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
        folder_path: PathBuf,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    Decrypt {
        encrypted_file: PathBuf,

        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: PathBuf,
    },
}
