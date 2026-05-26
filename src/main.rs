use std::path::PathBuf;

mod decrypt;
mod encrypt;

fn main() {
    let password = "very_good_password";

    let home = std::env::var("HOME").expect("Could not find HOME directory");
    let base_path = format!("{}/dashboard/codding/tests", home);

    // encrypt test
    let encrypt_test_folder = PathBuf::from(format!("{}/crypt", base_path));
    let encrypt_output_path = PathBuf::from(format!("{}/results/crypt.tar.gz.age", base_path));

    match encrypt::encrypt_folder(
        encrypt_test_folder,
        password.to_string(),
        Some(encrypt_output_path),
    ) {
        Ok(_) => println!("encryption test ok"),
        Err(e) => println!("encrypt test failed: {}", e),
    }

    // decrypt test
    let decrypt_test_folder = PathBuf::from(format!("{}/results/crypt.tar.gz.age", base_path));
    let decrypt_output_path = PathBuf::from(format!("{}/results/cryptDecrypted/", base_path));

    match decrypt::decrypt_folder(
        decrypt_test_folder,
        password.to_string(),
        decrypt_output_path,
    ) {
        Ok(_) => println!("decryption test ok"),
        Err(e) => println!("decryption test failed: {}", e),
    }
}
