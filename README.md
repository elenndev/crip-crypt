I use this to compress/decompress and encrypt/decrypt folders

## Installation and uninstall

```bash
cargo install --git https://github.com/elenndev/crip-crypt.git
cargo uninstall crip-crypt
```

## Usage

### Encrypt a folder
Compresses and encrypts an entire directory into a single `.tar.gz.age` file.

```bash
crip-crypt encrypt <folder_path> <output_directory> --password <password>
```

Example:

```bash
crip-crypt encrypt ./documents ./backups --password good-password
```

```text
folder-backup-YYYY-MM-DD_HH-MM-SS.tar.gz.age
```

example output:
```text
backups/
└── folder-backup-2026-05-30_18-42-15.tar.gz.age
```

---
### Decrypt a backup
Decrypts and extracts a previously generated backup.

```bash
crip-crypt decrypt <encrypted_file> <output_directory> --password <password>
```

example:
```bash
crip-crypt decrypt \
    ./backups/folder-backup-2026-05-30_18-42-15.tar.gz.age \
    ./restored \
    --password good-password
```

