use clap::{Parser, Subcommand};

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

    match cli.commands {
        Commands::Encrypt { file } => {
            println!("file name: {}", file)
        },

        Commands::Decrypt { file } => {
            println!("file name: {}", file)
        }
    }
}
