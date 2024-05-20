//! Functions to provide the base config for the entire program
//! (e.g. logging conf, server conf)

use dotenvy::dotenv;

use super::{
	env::{get_log_level_or_panic, get_log_type_or_panic, get_typed_env_or_panic},
	simple_con_log::con_info,
};
use std::{path::PathBuf, sync::OnceLock};

pub const FILE_PATH_PREFIX: &str = "file://";

/// Type of the logging to be provided by the tracing crate
#[derive(Debug, Clone)]
pub enum LogType {
	NONE,
	CONSOLE,
	FILE(PathBuf),
}

/// The base config ("the config necessary for starting the program")
/// for the entire program (http server config, log config)
#[derive(Debug, Clone)]
pub struct BaseConfig {
	pub port: u16,
	pub base_url: String,
	pub only_localhost: bool,
	pub log_type: LogType,
	pub log_level: tracing::Level,
}

static BASE_CONFIG: OnceLock<BaseConfig> = OnceLock::new();

/// Returns the already loaded base config or tries to load it
///
/// # Panics
/// Invalid types/values for config (but only on first run, afterwards it's guaranteed to not panic)
pub fn get_or_load_base_config() -> &'static BaseConfig {
	BASE_CONFIG.get_or_init(|| load_config())
}

/// Loads the base config from the values stored in the environment variables.
/// Also uses available `.env` files to populate the environment variables.
///
/// # Panics
/// Invalid types/values for config.
fn load_config() -> BaseConfig {
	let mut conf = BaseConfig {
		port: 8000,
		base_url: "/".to_string(),
		only_localhost: false,
		log_type: LogType::CONSOLE,
		log_level: tracing::Level::INFO,
	};

	if let Ok(pb) = dotenv() {
		match pb.to_str() {
			None => println!("[SOFT ERROR] Loaded .env file, but couldn't determine its path."),
			Some(path_str) => con_info(&format!("Loaded .env file \"{}\"", path_str)),
		}
	}

	conf.port = get_typed_env_or_panic::<u16>("PORT", conf.port);
	conf.base_url = get_typed_env_or_panic::<String>("BASE_URL", conf.base_url);
	conf.only_localhost = get_typed_env_or_panic::<bool>("ONLY_LOCALHOST", conf.only_localhost);
	conf.log_type = get_log_type_or_panic(&conf.log_type);
	conf.log_level = get_log_level_or_panic(&conf.log_level);

	conf
}
