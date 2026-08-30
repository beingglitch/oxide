mod data;
mod cipher;

use clap::{Parser, Subcommand};

use crate::{cipher::{encrypt, decrypt}, data::{get_data, set_data}};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        file: String
    },
    Decrypt {
        file: String
    }
}


fn main() {
    let cli = Cli::parse();
    let key: u8 = 42;

    match cli.commands {
        Commands::Encrypt { file } => {
            println!("file name: {}", file);
            let data = get_data(file).unwrap();
            set_data(&encrypt(&data, &key), Some("encrypted.txt".to_string())).unwrap();
        },

        Commands::Decrypt { file } => {
            println!("file name: {}", file);
            let encrypted_data = get_data(file).unwrap();
            set_data(&decrypt(&encrypted_data, &key), Some("decrypted.txt".to_string())).unwrap();
        }
    }
}
