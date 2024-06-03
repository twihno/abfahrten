use thiserror::Error;

#[derive(Error, Debug)]
#[error("a http error ocurred: (code: {code}, msg: {msg}")]
pub struct HttpError {
	pub code: u16,
	pub msg: String,
}
