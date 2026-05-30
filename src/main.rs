mod cli;
mod decrypt;
mod encrypt;
mod result;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() -> result::Result<()> {
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
