// mod cli;
mod error;
// mod crypto;
mod file_ops;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

fn main() {
let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { input, output, password } => {
            println!("🔒 Encrypting file...");
            println!("   Input: {:?}", input);
            println!("   Output: {:?}", output);
            println!("   Password length: {}", password.len());
        }

        Commands::Decrypt { input, output, password } => {

        }
    }
}
