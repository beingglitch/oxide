// mod cli;
mod error;
mod crypto;
mod file_ops;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use error::OxideError;

#[derive(Parser)]
#[command(name = "oxide")]
#[command(about = "A blazingly fast file encryption tool", long_about = None)]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(short, long)]
        password: String
    },
    Decrypt {
        #[arg(short, long)]
        input: PathBuf,

        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        password: String
    }
}

fn main() -> Result<(), OxideError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output, password } => {
            
            let data = file_ops::read_file(&input)?;

            let encrypted_data = crypto::encrypt(&data, &password)?;

            let out_path = match output {
                Some(p) => p,
                None => {
                    let mut p = input.clone();

                    let new_name = match p.file_name().and_then(std::ffi::OsStr::to_str) {
                        Some(name) => format!("{}.oxide", name),
                        None => String::from("output.oxide"),
                    };

                    p.set_file_name(new_name);
                    p
                }
            };

            file_ops::write_file(&out_path, &encrypted_data)?;

            println!("Encrypted -> {}", out_path.display());
        }

        Commands::Decrypt { input, output, password } => {

            match input.extension().and_then(std::ffi::OsStr::to_str) {
                Some("oxide") => {} // good
                _ => return Err(OxideError::InvalidFormatError),
            }


            let data = file_ops::read_file(&input)?;

            let decrypted_data = crypto::decrypt(&data, &password)?;

            let out_path = match output {
                Some(p) => p,
                None => {
                    let mut p = input.clone();

                    // remove the extension (set_extension("") removes it)
                    p.set_extension("");

                    // If removing extension produced an empty filename (edge cases),
                    // fall back to a safe "output.dec" name:
                    if p.file_name().is_none() {
                        p.set_file_name("output.dec");
                    }
                    p
                }
            };

            file_ops::write_file(&out_path, &decrypted_data)?;

            println!("Decrypted File -> {}", out_path.display());
        }
    }

    Ok(())
}
