pub fn show_help() {
    println!(
        r#"Crip Crypt

USAGE:
    crip-crypt <COMMAND> [OPTIONS]

COMMANDS:
    encrypt    Compress and encrypt a folder
    decrypt    Decrypt and restore a backup

ENCRYPT:
    crip-crypt encrypt <folder_path> <output_directory> --password <password>

    Aliases:
        -p, --password

DECRYPT:
    crip-crypt decrypt <encrypted_file> <output_directory> --password <password>

    Aliases:
        -p, --password

EXAMPLES:
    crip-crypt encrypt ./documents ./backups --password my-secret

    crip-crypt decrypt \
        ./backups/folder-backup-2026-05-30_18-42-15.tar.gz.age \
        ./restored \
        --password my-secret
"#
    );
}

pub fn show_encrypt_help() {
    println!(
        r#"USAGE:
    crip-crypt encrypt <folder_path> <output_directory> --password <password>

DESCRIPTION:
    Compresses a folder into a tar.gz archive and encrypts it using AGE.

EXAMPLE:
    crip-crypt encrypt ./documents ./backups --password my-secret
"#
    );
}

pub fn show_decrypt_help() {
    println!(
        r#"USAGE:
    crip-crypt decrypt <encrypted_file> <output_directory> --password <password>

DESCRIPTION:
    Decrypts an AGE-encrypted archive and restores the original folder.

EXAMPLE:
    crip-crypt decrypt backup.tar.gz.age ./restored --password my-secret
"#
    );
}
