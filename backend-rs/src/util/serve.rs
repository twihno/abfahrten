use axum::{response::Html, routing::get, Router};
use tracing::info;

use crate::util::exit::exit_critical;

use super::base_config::BaseConfig;

pub async fn start_server(config: &BaseConfig) {
	// build our application with a route
	let app = Router::new().route("/", get(handler));

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

	info!("listening on {}", listener.local_addr().unwrap());

	axum::serve(listener, app)
		.await
		.unwrap_or_else(|_| exit_critical("Failed to start webserver", true));
}

async fn handler() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}
