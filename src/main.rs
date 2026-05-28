mod cli;
mod decrypt;
mod encrypt;

use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt {
            folder_path,
            password,
            output,
        } => encrypt::encrypt_folder(folder_path, password, output)?,
        Commands::Decrypt {
            encrypted_file,
            password,
            output,
        } => decrypt::decrypt_folder(encrypted_file, password, output)?,
    }

    Ok(())
}
