use std::sync::{Arc, Mutex};

use abfahrten_backend::util::{
	base_config::get_or_load_base_config,
	exit::exit_critical,
	serve::start_server,
	service_config::{self, load_and_validate_service_config},
	simple_con_log::con_info,
	tracing::init_tracing,
};
use display_error_chain::ErrorChainExt;

#[tokio::main]
async fn main() {
	con_info("YOLO\n\nfdsf")
	// let base_config = get_or_load_base_config();
	// init_tracing(&base_config);

	// let service_config = match load_and_validate_service_config() {
	// 	Err(err) => exit_critical(&err.into_chain().to_string(), true),
	// 	Ok(val) => Arc::new(Mutex::new(val)),
	// };

	// start_server(base_config, service_config.clone()).await;
}
