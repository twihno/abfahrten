use abfahrten_backend::util::{base_config::get_or_load_base_config, serve::start_webserver};

#[tokio::main]
async fn main() {
	let base_config = get_or_load_base_config();

	tokio::spawn(start_webserver(base_config));
}
