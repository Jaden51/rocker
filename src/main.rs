use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(name = "rocker")]
#[command(about = "Docker in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { cmd: String },
    Images,
    Ps,
    
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { cmd } => {
            run_container(cmd);
        }
        Commands::Images => {
            list_images();
        }
        Commands::Ps => {
            show_containers();
        }
    }
}

fn run_container(cmd: &String) {
    println!("command: {}", cmd)
}

fn list_images() {
    println!("Listing images");
}

fn show_containers() {
    println!("Showing containers");
}
