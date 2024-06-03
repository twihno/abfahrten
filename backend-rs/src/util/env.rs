//! Helper functions for working with environment variables (or at least get their values)

use std::{
	any::type_name,
	env::{self, VarError},
	path::PathBuf,
	str::FromStr,
};

use super::{
	base_config::{LogType, FILE_PATH_PREFIX},
	exit::exit_critical,
};

use tracing::Level;

/// Errors that can occur during the reading and parsing of
/// an environment variable
#[derive(thiserror::Error, Debug)]
pub enum EnvError {
	#[error("Environment variable \"{variable_name}\" not present")]
	/// Environment variable not present
	NotPresent { variable_name: String },
	#[error("The value of the environment variable \"{variable_name}\" is not valid unicode")]
	/// Environment variable not valid unicode
	NotUnicode { variable_name: String },
	#[error("Error while trying to parse the environment variable \"{variable_name}\" as the requested type \"{r#type}\"")]
	/// Error while trying to parse the environment variable as the requested type
	ParseError {
		variable_name: String,
		r#type: String,
	},
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type.
pub fn get_typed_env<T: FromStr>(key: &str) -> Result<T, EnvError> {
	let env_value = match env::var(key) {
		Err(err) => match err {
			VarError::NotPresent => {
				return Err(EnvError::NotPresent {
					variable_name: key.to_string(),
				})
			}
			VarError::NotUnicode(_) => {
				return Err(EnvError::NotUnicode {
					variable_name: key.to_string(),
				})
			}
		},
		Ok(val) => val,
	};

	match env_value.parse::<T>() {
		Err(_) => Err(EnvError::ParseError {
			variable_name: key.to_string(),
			r#type: type_name::<T>().to_string(),
		}),
		Ok(val) => Ok(val),
	}
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type.
///
/// If the environment variable isn't set, it returns the default value.
pub fn get_typed_env_with_default<T: FromStr>(key: &str, default: T) -> Result<T, EnvError> {
	let err = match get_typed_env::<T>(key) {
		Ok(val) => return Ok(val),
		Err(err) => err,
	};

	match err {
		EnvError::NotPresent { .. } => return Ok(default),
		EnvError::NotUnicode { .. } => return Err(err),
		EnvError::ParseError { .. } => return Err(err),
	}
}

/// Tries to retrieve the environment variable `key` and convert it to
/// the desired type. If the variable isn't set, the default value
/// will be returned.
///
/// # Panics
/// - `NotUnicode`: Unicode error. Inherited from `std::env::var`
/// - `ParseError`: Parsing error:
/// 	Couldn't convert the environment variable to the desired type
pub fn get_typed_env_or_panic<T: FromStr>(key: &str, default: T) -> T {
	let env_val = get_typed_env(key);

	match env_val {
		Ok(val) => return val,
		Err(err) => match err {
			EnvError::NotPresent { .. } => return default,
			EnvError::NotUnicode { .. } => exit_critical(
				&format!(
					"Value for environment variable \"{}\" is not valid unicode.",
					key
				),
				true,
			),
			EnvError::ParseError { .. } => exit_critical(
				&format!("Couldn't parse environment variable \"{}\".", key),
				true,
			),
		},
	};
}

/// Retrieves the log_type from the environment variable "LOG"
///
/// If "LOG" isn't set it falls back to the provided default value.
///
/// # Panics
/// - Invalid path (conversion to PathBuf failed)
pub fn get_log_type_or_panic(default: &LogType) -> LogType {
	// Catch empty environment variable -> Fallback to default
	let env_log = match env::var("LOG") {
		Err(_) => return default.clone(),
		Ok(val) => val,
	};

	if let Some(path) = env_log.strip_prefix(FILE_PATH_PREFIX) {
		match PathBuf::from_str(path) {
			Ok(path_as_pathbuf) => return LogType::FILE(path_as_pathbuf),
			Err(e) => exit_critical(
				&format!(
					"The following error occurred while parsing the logging path: ${}",
					e
				),
				true,
			),
		}
	};

	match env_log.to_lowercase().as_ref() {
		"console" => LogType::CONSOLE,
		"none" => LogType::NONE,
		"off" => LogType::NONE,
		_ => default.clone(),
	}
}

/// Retrieves the log_level from the environment variable "LOG_LEVEL".
///
/// If "LOG_LEVEL" isn't set it falls back to the provided default value.
/// It is case-insensitive and accepts the following values:
/// - `error`
/// - `warn`
/// - `info`
/// - `debug`
/// - `trace`
///
/// # Panics
/// - Invalid value in environment variable
pub fn get_log_level_or_panic(default: &Level) -> Level {
	// Catch empty environment variable -> Fallback to default
	let env_level = match env::var("LOG_LEVEL") {
		Err(_) => return default.clone(),
		Ok(val) => val,
	};

	match env_level.to_lowercase().as_ref(){
		"error"=> Level::ERROR,
		"warn"=> Level::WARN,
		"info" => Level::INFO,
		"debug" => Level::DEBUG,
		"trace"=> Level::TRACE,
		_ => exit_critical(&format!(
			"Invalid value for LOG_LEVEL: ${}. Possible values are: \"error\", \"warn\", \"info\", \"debug\" and \"trace\" ",env_level),true)
	}
}
