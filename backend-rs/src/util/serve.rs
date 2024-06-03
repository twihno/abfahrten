use std::sync::{Arc, Mutex};

use tracing::info;

use crate::util::{endpoints::create_router, exit::exit_critical};

use super::{base_config::BaseConfig, service_config::ServiceConfig};

pub async fn start_server(base_config: &BaseConfig, service_config: Arc<Mutex<ServiceConfig>>) {
	// build our application with a route
	let app = create_router();

	let host = if base_config.only_localhost {
		"localhost"
	} else {
		"0.0.0.0"
	};

	// run it
	let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, base_config.port))
		.await
		.unwrap_or_else(|_| {
			exit_critical(
				&format!("Failed to bind to {}:{}", host, base_config.port),
				true,
			);
		});

	info!("listening on {}", listener.local_addr().unwrap());

	axum::serve(listener, app)
		.await
		.unwrap_or_else(|_| exit_critical("Failed to start webserver", true));
}
