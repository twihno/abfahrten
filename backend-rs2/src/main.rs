use std::sync::Arc;

use clap::{Parser, Subcommand};

use backend_rs2::{
    api::start_server, clients::transport::get_all_available_providers, load_config,
    search_stations,
};
use nu_ansi_term::Style;
use tokio::task::JoinSet;
use tracing::debug;

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

        /// Detailed output
        #[arg(short, long)]
        detailed: bool,
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

    match cli.command {
        // Start the cache server with the environment config
        Commands::Start => {
            let mut set = JoinSet::new();

            let config = Arc::new(load_config());
            debug!("{config:?}");

            set.spawn(start_server(config));

            set.join_all().await;
        }
        // Search for a station with a specified provider
        Commands::Search {
            name,
            provider,
            detailed,
        } => {
            search_stations(&name, &provider, detailed).await;
        }
        // Do stuff with the available providers, e.g. list them
        Commands::Providers { list } => {
            if list {
                println!(
                    "{}",
                    Style::new()
                        .bold()
                        .underline()
                        .paint("Available providers:")
                );
                for (id, name) in get_all_available_providers() {
                    println!(" - {id} / {name}");
                }
            }
        }
    };
}
