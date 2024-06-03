/// Configuration for the provided services
use thiserror::Error;

use super::env::{get_typed_env, get_typed_env_with_default, EnvError};

#[derive(Debug, Clone)]
pub struct ServiceConfig {
	pub station_identifiers: Vec<String>,
	pub use_first_result: bool,
	pub fetch_interval: u16,
}

#[derive(Error, Debug)]
pub enum ServiceConfigLoadError {
	#[error("The list of stations is empty")]
	NoStationsGiven,
	#[error("Error while loading/parsing the environment variable")]
	EnvError { source: EnvError },
}

impl From<EnvError> for ServiceConfigLoadError {
	fn from(value: EnvError) -> Self {
		ServiceConfigLoadError::EnvError { source: value }
	}
}

pub fn load_and_validate_service_config() -> Result<ServiceConfig, ServiceConfigLoadError> {
	let config = match load_service_config() {
		Ok(val) => val,
		Err(err) => return Err(err),
	};

	Ok(config)
}

/// Tries to load the config relevant to the service from the environment
fn load_service_config() -> Result<ServiceConfig, ServiceConfigLoadError> {
	let mut config = ServiceConfig {
		station_identifiers: vec![],
		use_first_result: false,
		fetch_interval: 20,
	};

	config.station_identifiers = match get_typed_env::<String>("STATIONS") {
		Ok(val) => val
			.split(";")
			.map(|str_elem| str_elem.to_string())
			.collect(),
		Err(err) => return Err(ServiceConfigLoadError::from(err)),
	};

	if config.station_identifiers.len() <= 0 {
		return Err(ServiceConfigLoadError::NoStationsGiven);
	}

	config.use_first_result =
		match get_typed_env_with_default::<bool>("USE_FIRST_RESULT", config.use_first_result) {
			Ok(val) => val,
			Err(err) => return Err(ServiceConfigLoadError::from(err)),
		};

	config.fetch_interval =
		match get_typed_env_with_default::<u16>("FETCH_INTERVAL", config.fetch_interval) {
			Ok(val) => val,
			Err(err) => return Err(ServiceConfigLoadError::from(err)),
		};

	Ok(config)
}
