use std::process::exit;

use clients::transport::get_new_provider_by_id;
use dotenvy::dotenv;
use envconfig::Envconfig;
use tracing::{debug, error, info};
use util::{config::AppConfig, tracing::init_tracing};

pub mod clients;
pub mod util;

pub fn start_server() -> AppConfig {
    let env_load_result = dotenv();

    init_tracing();

    match env_load_result {
        Ok(value) => {
            info!("Using .env file {value:?}")
        }
        Err(err) => {
            debug!("Failed to load .env file: \"{err}\"");
        }
    }

    let config = AppConfig::init_from_env().unwrap_or_else(|err| {
        error!("{err}");
        exit(1);
    });

    config
}

/// Search for the matching stations with the given provider
pub async fn search_stations(name: &str, provider_id: &str) {
    if name.is_empty() {
        println!("ERROR: Empty station name");
        exit(1);
    }

    if name.is_empty() {
        println!("ERROR: Empty provider");
        exit(1);
    }

    let Some(provider) = get_new_provider_by_id(provider_id) else {
        println!("ERROR: Provider \"{provider_id}\" doesn't exist");
        exit(1);
    };

    println!("Searching for \"{name}\" with \"{provider_id}\"");

    let station_list = match provider.search_station(name).await {
        Ok(value) => value,
        Err(err) => {
            println!("Error, couldn't get stations");
            println!("{err}");
            exit(1)
        }
    };

    for station in &station_list {
        println!("{station}");
    }
}
