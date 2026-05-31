mod cli;
mod decrypt;
mod encrypt;
mod help;
mod result;

use crate::cli::Command;

fn main() -> result::Result<()> {
    let command =
        cli::parse_args().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;

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
