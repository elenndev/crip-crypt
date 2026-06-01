use age::{Encryptor, secrecy::SecretString};
use chrono::Local;
use flate2::{Compression, write::GzEncoder};
use std::{fs::File, io::BufWriter, path::PathBuf};
use tar::Builder;

use crate::result;

// use streaming encryption
pub fn encrypt_folder(
    folder_path: PathBuf,
    password: String,
    output: PathBuf,
) -> result::Result<()> {
    if !folder_path.exists() {
        return Err(format!(
            "Input folder path does not exist: {}\n",
            folder_path.display()
        )
        .into());
    }

    if !output.exists() {
        return Err(format!("Output folder path does not exist: {}", output.display()).into());
    }

    if output.is_file() {
        return Err(format!("Output must be a directory: {}", output.display()).into());
    }

    let output_file = {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        output.join(format!("folder-backup-{}.tar.gz.age", timestamp))
    };

    println!("Encrypting...");
    println!("Input : {:?}", folder_path);
    println!("Output: {:?}", output_file);

    // build output file and file writter
    let file = File::create(&output_file)?;
    let writer = BufWriter::new(file);

    // set encryption
    let encryptor = Encryptor::with_user_passphrase(SecretString::new(password.into()));
    let age_writer = encryptor.wrap_output(writer)?;

    // set compression maker
    let gzip_encoder = GzEncoder::new(age_writer, Compression::default());

    // tar builder
    let mut tar_builder = Builder::new(gzip_encoder);
    tar_builder.append_dir_all(".", &folder_path)?;

    // final piece, flush
    tar_builder.finish()?; // close tar layer, write final tar blocks, 
    // tar finished but not dropped, still owns encoder

    let gzip_encoder = tar_builder.into_inner()?; // get back ownership
    let age_writer = gzip_encoder.finish()?; // consume encoder and destroy it, and write footer
    // footer = checksum + original size

    age_writer.finish()?; // close age writer and add encryption close tag

    println!("folder encrypted successfully.");

    Ok(())
}
