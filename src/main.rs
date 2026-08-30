mod data;
mod cipher;

use clap::{Parser, Subcommand};

use crate::{cipher::{decrypt, encrypt}, data::{get_data, set_data}};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        file: String,
        #[arg(short, long)]
        password: String
    },
    Decrypt {
        file: String,
        #[arg(short, long)]
        password: String
    }
}


fn main() {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Encrypt { file, password } => {
            println!("file name: {}", file);
            let data = get_data(file).unwrap();
            set_data(&encrypt(&data, &password), Some("encrypted.txt".to_string())).unwrap();
        },

        Commands::Decrypt { file, password } => {
            println!("file name: {}", file);
            let encrypted_data = get_data(file).unwrap();
            match decrypt(&encrypted_data, &password) {
                Ok(data) => {
                    set_data(&data, Some("decrypted.txt".to_string())).unwrap()
                },
                Err(err) => {
                    println!("{err}")
                }
            }
            
        }
    }
}
