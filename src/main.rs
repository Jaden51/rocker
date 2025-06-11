mod commands;
mod containers;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rocker")]
#[command(about = "Containerization in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { cmd_args: Vec<String> },
    Images,
    Ps,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { cmd_args } => {
            commands::run::run(cmd_args);
        }
        Commands::Images => {
            commands::list::list();
        }
        Commands::Ps => {
            commands::ps::ps();
        }
    }
}
