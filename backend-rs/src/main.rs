use abfahrten_backend::util::{
	base_config::get_or_load_base_config, serve::start_server, tracing::init_tracing,
};

#[tokio::main]
async fn main() {
	let base_config = get_or_load_base_config();

	init_tracing(&base_config);

	start_server(base_config).await;
}
