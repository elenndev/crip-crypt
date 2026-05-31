use std::{env, path::PathBuf};

use crate::help::{show_decrypt_help, show_encrypt_help, show_help};

pub fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("encrypt") => parse_encrypt(&args),
        Some("decrypt") => parse_decrypt(&args),
        Some("--help") | Some("-h") => parse_help(&args),
        _ => Err("Invalid Command, use --help".into()),
    }
}

pub fn parse_encrypt(args: &[String]) -> Result<Command, String> {
    if args.len() != 6 {
        return Err("Usage: cript-crypt encrypt <folder <output> --pasword <password>".into());
    }

    if args[4] != "--password" && args[4] != "-p" {
        return Err("Missing --password flag".into());
    }

    Ok(Command::Encrypt {
        folder_path: PathBuf::from(&args[2]),
        output: PathBuf::from(&args[3]),
        password: args[5].clone(),
    })
}

pub fn parse_decrypt(args: &[String]) -> Result<Command, String> {
    if args.len() != 6 {
        return Err("Usage: cript-crypt decrypt <folder> <output> --pasword <password>".into());
    }
    if args[4] != "--password" && args[4] != "-p" {
        return Err("Missing --password flag".into());
    }

    Ok(Command::Decrypt {
        encrypted_file: PathBuf::from(&args[2]),
        output: PathBuf::from(&args[3]),
        password: args[5].clone(),
    })
}

fn parse_help(args: &[String]) -> Result<Command, String> {
    if args.len() != 3 && args.len() != 4 {
        return Err("Usage: cript-crypt --help\n cript-crypt --help <command>\n Use --help to see commands list".into());
    }

    if args.len() == 3 {
        show_help();
    }

    if args.len() == 4 {
        match args[3].as_str() {
            "encrypt" => show_encrypt_help(),
            "decrypt" => show_decrypt_help(),

            command => {
                eprintln!(
                    "Invalid command: {}\n Use --help to see commands list",
                    command
                );
                std::process::exit(1);
            }
        }
    }

    std::process::exit(0);
}

pub enum Command {
    Encrypt {
        // Positional arguments
        folder_path: PathBuf,
        output: PathBuf,

        // --password
        password: String,
    },

    Decrypt {
        // Positional arguments
        encrypted_file: PathBuf,
        output: PathBuf,

        // --password
        password: String,
    },
}
