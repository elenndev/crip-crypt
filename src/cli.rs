use std::{env, path::PathBuf};

use crate::help::{show_decrypt_help, show_encrypt_help, show_help};

pub fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("encrypt") => parse_encrypt(&args),
        Some("decrypt") => parse_decrypt(&args),
        Some("--help") | Some("-h") => parse_help(&args),
        _ => Err("Error: Invalid Command, use --help".into()),
    }
}

pub fn parse_encrypt(args: &[String]) -> Result<Command, String> {
    if args.len() != 6 {
        return Err(format!(
            r#"Error trying to use encrypt:
        'cript-crypt {}' is an invalid command

aUsage: cript-crypt encrypt <folder <output> --pasword <password>

        Use --help to see commands help list"#,
            args.join(" ")
        ));
    }

    if args[4] != "--password" && args[4] != "-p" {
        return Err(format!(
            r#"Error: 
        command 'cript-crypt {}' missing --password flag

        Usage: cript-crypt decrypt <folder> <output> --pasword <password>

        Use --help to see commands help list"#,
            args.join(" ")
        ));
    }

    Ok(Command::Encrypt {
        folder_path: PathBuf::from(&args[2]),
        output: PathBuf::from(&args[3]),
        password: args[5].clone(),
    })
}

pub fn parse_decrypt(args: &[String]) -> Result<Command, String> {
    if args.len() != 6 {
        return Err(format!(
            r#"Error trying to use decrypt:
        'cript-crypt {}' is an invalid command

        Usage: cript-crypt decrypt <folder> <output> --pasword <password>

        Use --help to see commands help list"#,
            args.join(" ")
        ));
    }
    if args[4] != "--password" && args[4] != "-p" {
        return Err("Error trying to use decrypt: Missing --password flag".into());
    }

    Ok(Command::Decrypt {
        encrypted_file: PathBuf::from(&args[2]),
        output: PathBuf::from(&args[3]),
        password: args[5].clone(),
    })
}

fn parse_help(args: &[String]) -> Result<Command, String> {
    if args.len() != 2 && args.len() != 3 {
        return Err(r#"Error trying to show help
        Usage: cript-crypt --help
        cript-crypt --help <command> Use --help to see commands list"#
            .into());
    }

    if args.len() == 2 {
        show_help();
    }

    if args.len() == 3 {
        match args[2].as_str() {
            "encrypt" => show_encrypt_help(),
            "decrypt" => show_decrypt_help(),

            command => {
                eprintln!(
                    "Error: Invalid command: '{}' Use --help to see commands list",
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
