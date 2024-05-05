use std::{
	env::{self, VarError},
	str::FromStr,
};

use super::exit::exit_critical;

#[derive(Debug)]
pub enum EnvError<T> {
	NotPresent,
	NotUnicode,
	ParseError(T),
}

pub fn get_typed_env<T: FromStr>(key: &str) -> Result<T, EnvError<<T as FromStr>::Err>> {
	let env_value = match env::var(key) {
		Err(err) => match err {
			VarError::NotPresent => return Err(EnvError::NotPresent),
			VarError::NotUnicode(_) => return Err(EnvError::NotUnicode),
		},
		Ok(val) => val,
	};

	match env_value.parse::<T>() {
		Err(err) => Err(EnvError::ParseError(err)),
		Ok(val) => Ok(val),
	}
}

pub fn get_typed_env_or_panic<T: FromStr>(key: &str, default: T) -> T {
	let env_val = get_typed_env(key);

	match env_val {
		Ok(val) => return val,
		Err(err) => match err {
			EnvError::NotPresent => return default,
			EnvError::NotUnicode => exit_critical(
				&format!(
					"Value for environment variable \"{}\" is not valid unicode.",
					key
				),
				true,
			),
			EnvError::ParseError(_) => exit_critical(
				&format!("Couldn't parse environment variable \"{}\".", key),
				true,
			),
		},
	};
}
