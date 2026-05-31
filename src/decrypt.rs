use age::Decryptor;
use age::scrypt;
use age::secrecy::SecretString;
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use tar::Archive;

use crate::result;

pub fn decrypt_folder(
    encrypted_file: PathBuf,
    password: String,
    output: PathBuf,
) -> result::Result<()> {
    if !encrypted_file.exists() {
        return Err(format!("Encrypted file not found: {}\n", encrypted_file.display()).into());
    }

    println!("Decrypting...");
    println!("Input : {:?}", encrypted_file);
    println!("Output: {:?}", output);

    // open encrypted file
    let input = File::open(encrypted_file)?;
    // read chunks
    let reader = BufReader::new(input);

    // set decryptor
    let decryptor = Decryptor::new(reader)?;
    // create an descryption key from the password
    let identity = scrypt::Identity::new(SecretString::new(password.into_boxed_str()));
    // perform decryption
    let decrypted_reader = decryptor.decrypt(std::iter::once(&identity as &dyn age::Identity))?;

    // decompression
    let gzip_encoder = GzDecoder::new(decrypted_reader);
    // wraps the gzip decoder with Tar capabilities
    let mut archive = Archive::new(gzip_encoder);
    // read tar headers and file contents, create directories and files at the output
    archive.unpack(output)?;
    println!("folder restored successfully.");

    Ok(())
}
