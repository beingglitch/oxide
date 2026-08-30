mod data;
mod cipher;
mod errors;

use clap::{Parser, Subcommand};

use crate::{cipher::{decrypt, encrypt}, data::{get_data, set_data}, errors::OxideError};


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


fn main() -> Result<(), OxideError> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Encrypt { file, password } => {
            println!("file name: {}", file);
            let data = get_data(file)?;
            set_data(&encrypt(&data, &password), Some("encrypted.txt".to_string()))?;
        },

        Commands::Decrypt { file, password } => {
            println!("file name: {}", file);
            let encrypted_data = get_data(file)?;
            set_data(&decrypt(&encrypted_data, &password)?, Some("decrypted.txt".to_string()))?;
        }
    }

    Ok(())
}
