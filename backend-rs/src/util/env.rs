use std::{
	env::{self, VarError},
	str::FromStr,
};

#[derive(Debug)]
pub enum EnvError<T> {
	NotPresent,
	NotUnicode,
	ParseError(T),
}

pub fn get_typed_env<T: FromStr>(key: String) -> Result<T, EnvError<<T as FromStr>::Err>> {
	let env_value = match env::var(key) {
		Err(err) => match err {
			VarError::NotPresent => return Err(EnvError::NotPresent),
			VarError::NotUnicode(_) => return Err(EnvError::NotUnicode),
		},
		Ok(val) => val,
	};

	print!("{}", env_value);

	match env_value.parse::<T>() {
		Err(err) => Err(EnvError::ParseError(err)),
		Ok(val) => Ok(val),
	}
}
