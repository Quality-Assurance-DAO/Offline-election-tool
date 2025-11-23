//! CLI binary entry point for the Offline NPoS Election Tool

use clap::Parser;
use offline_election::cli::commands::{RunCommand, ServerCommand};

#[derive(Parser)]
#[command(name = "offline-election")]
#[command(about = "Offline NPoS Election Tool - Run election simulations offline")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run an election simulation
    Run(RunCommand),
    /// Start the REST API server
    Server(ServerCommand),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Command::Run(cmd) => {
            if let Err(e) = cmd.execute().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Command::Server(cmd) => {
            if let Err(e) = cmd.execute().await {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

