use thiserror::Error;

pub struct Station {
	pub name: String,
	pub id: String,
}

#[derive(Error, Debug)]
pub enum StationAddError {
	#[error("Station not found")]
	StationNotFound,
	#[error("Multiple Stations available. Not possible to determine the desired station.")]
	MultipleStationsFound,
}
