mod cli;
mod decrypt;
mod encrypt;
mod help;
mod result;

use crate::cli::Command;

fn main() -> result::Result<()> {
    let command = cli::parse_args().unwrap_or_else(|err| {
        eprintln!("{err}");
        std::process::exit(1);
    });

    match command {
        Command::Encrypt {
            folder_path,
            password,
            output,
        } => encrypt::encrypt_folder(folder_path, password, output)?,
        Command::Decrypt {
            encrypted_file,
            password,
            output,
        } => decrypt::decrypt_folder(encrypted_file, password, output)?,
    }

    Ok(())
}
