use super::env::get_typed_env;

pub struct Config {
	port: u8,
	base_url: String,
}

pub fn load_config() -> Result<Config, String> {
	let mut conf = Config {
		port: 3,
		base_url: "".to_string(),
	};

	get_typed_env::<u8>("Test".to_string());

	conf.port = 3;

	Ok(conf)
}
