use super::env::get_typed_env_or_panic;
use std::sync::OnceLock;

#[derive(Debug)]
pub struct BaseConfig {
	port: u16,
	base_url: String,
}
static BASE_CONFIG: OnceLock<BaseConfig> = OnceLock::new();

pub fn get_or_load_base_config() -> &'static BaseConfig {
	BASE_CONFIG.get_or_init(|| load_config())
}

fn load_config() -> BaseConfig {
	let mut conf = BaseConfig {
		port: 8000,
		base_url: "/".to_string(),
	};

	conf.port = get_typed_env_or_panic::<u16>("port", conf.port);
	conf.base_url = get_typed_env_or_panic::<String>("base_url", conf.base_url);

	conf
}
