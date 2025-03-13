use clap::{Parser, Subcommand};
use tracing::{debug, error, info};

use backend_rs2::{search_stations, start_server};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Simple program to greet a person
#[derive(Subcommand, Debug)]
enum Commands {
    /// Starts the application
    Start,
    /// Searches for a station
    Search {
        /// The name of the desired station
        #[arg(short, long)]
        name: String,

        /// The provider that should be used
        #[arg(short, long)]
        provider: String,
    },
    /// Actions related to the available providers
    Providers {
        /// List all available providers
        #[arg(short, long)]
        list: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        // Start the cache server with the environment config
        Commands::Start => {
            let config = start_server();

            println!("{config:?}");
        }
        // Search for a station with a specified provider
        Commands::Search { name, provider } => {
            search_stations(name, provider).await;
        }
        // Do stuff with the available providers, e.g. list them
        Commands::Providers { list } => {
            println!("Should I list? {list}")
        }
    };
}
