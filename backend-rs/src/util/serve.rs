use axum::{response::Html, routing::get, Router};

use crate::util::exit::exit_critical;

use super::base_config::BaseConfig;

pub async fn start_webserver(config: &BaseConfig) {
	// build our application with a route
	let app = Router::new().route("/", get(handler));

	// run it
	let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
		.await
		.unwrap();

	println!("listening on {}", listener.local_addr().unwrap());

	axum::serve(listener, app)
		.await
		.unwrap_or_else(|_| exit_critical("Failed to start webserver", true));
}

async fn handler() -> Html<&'static str> {
	Html("<h1>Hello, World!</h1>")
}
