use std::sync::Arc;

use app_state::AppState;
use router::create_router;
use tracing::info;

use crate::util::{config::AppConfig, exit::exit_critical};

pub mod app_state;
pub mod error;
pub mod router;
pub mod routes;

/// Initialize and start the webserver with the provided config
pub async fn start_server(config: Arc<AppConfig>) {
    let config = &config.server_config;

    let app_state = Arc::new(AppState {});

    // build our application with a route
    let app = create_router(app_state.clone());

    let host = if config.only_localhost {
        "localhost"
    } else {
        "0.0.0.0"
    };

    // run it
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, config.port))
        .await
        .unwrap_or_else(|_| {
            exit_critical(&format!("Failed to bind to {}:{}", host, config.port), true);
        });

    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| exit_critical("The internal webserver failed", true));
}
