mod cipher;
mod errors;
mod constants;

use std::{fs::File, io::{BufReader, BufWriter, Read, Write}};
use clap::{Parser, Subcommand};

use crate::{cipher::{decrypt, derive_seed, encrypt, generate_keystream}, constants::{BUFFER_SIZE, MAGIC}, data::{get_data, set_data}, errors::OxideError};


#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    commands: Commands
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        input_file: String,
        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: Option<String>
    },
    Decrypt {
        input_file: String,
        #[arg(short, long)]
        password: String,

        #[arg(short, long)]
        output: Option<String>
    }
}

fn main() -> Result<(), OxideError> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Encrypt { input_file, password, output } => {
            println!("file name: {}", input_file);

            let input_file = File::open(input_file).map_err(OxideError::ReadFailed)?;
            let mut reader = BufReader::new(input_file);
            let output_file = File::create(output.unwrap_or("encrypted_file.txt".to_string())).map_err(OxideError::WriteFailed)?;
            let mut writer = BufWriter::new(output_file);
            let mut buffer = [0u8; BUFFER_SIZE];

            let seed = derive_seed(&password.as_bytes());
            let mut pointer: usize = 0;

            // Add MAGIC
            let keystream: Vec<u8> = generate_keystream(&seed, MAGIC.len(), pointer);
            writer.write_all(&encrypt(&MAGIC.to_vec(), keystream)).map_err(OxideError::WriteFailed)?;
            pointer += MAGIC.len();

            loop {
                let bytes_read = reader.read(&mut buffer).map_err(OxideError::ReadFailed)?;

                if bytes_read == 0 { break; }

                let chunk = &buffer[0..bytes_read];

                let keystream: Vec<u8> = generate_keystream(&seed, bytes_read, pointer);
                pointer += bytes_read;
                writer.write_all(&encrypt(&chunk.to_vec(), keystream)).map_err(OxideError::WriteFailed)?;
            }
        },

        Commands::Decrypt { input_file, password , output} => {
                        println!("file name: {}", input_file);

            let input_file = File::open(input_file).map_err(OxideError::ReadFailed)?;
            let mut reader = BufReader::new(input_file);
            let output_file = File::create(output.unwrap_or("decrypted_file.txt".to_string())).map_err(OxideError::WriteFailed)?;
            let mut writer = BufWriter::new(output_file);
            let mut buffer = [0u8; BUFFER_SIZE];

            let seed = derive_seed(&password.as_bytes());
            let mut pointer: usize = 0;

            // Read MAGIC
            let mut magic_buffer = [0u8; MAGIC.len()];

            reader.read_exact(&mut magic_buffer).map_err(OxideError::ReadFailed)?;
            let keystream: Vec<u8> = generate_keystream(&seed, MAGIC.len(), pointer);
            if decrypt(&magic_buffer.to_vec(), keystream) != MAGIC {
                return Err(OxideError::WrongPassword);
            }

            pointer += MAGIC.len();

            loop {
                let bytes_read = reader.read(&mut buffer).map_err(OxideError::ReadFailed)?;

                if bytes_read == 0 { break; }

                let chunk = &buffer[0..bytes_read];

                let keystream: Vec<u8> = generate_keystream(&seed, bytes_read, pointer);
                pointer += bytes_read;
                writer.write_all(&decrypt(&chunk.to_vec(), keystream)).map_err(OxideError::WriteFailed)?;
            }
        }
    }

    Ok(())
}
