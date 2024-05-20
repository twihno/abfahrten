use thiserror::Error;

#[derive(Error, Debug)]
#[error("a http error ocurred: (code: {code}, msg: {msg}")]
pub struct HttpError {
	code: u8,
	msg: String,
}
