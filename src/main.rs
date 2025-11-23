//! CLI binary entry point for the Offline NPoS Election Tool

use clap::Parser;
use offline_election::cli::commands::RunCommand;

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
    // Server command will be added in Phase 8
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
    }
}

