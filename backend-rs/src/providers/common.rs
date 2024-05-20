pub mod departure;
pub mod station;

use chrono::TimeZone;
use reqwest::Client;
use thiserror::Error;

use self::departure::DepartureList;

pub struct Station {
	pub display_name: String,
	pub id: String,
}

#[derive(Error, Debug)]
pub enum StationAddError {
	#[error("Station not found")]
	StationNotFound,
	#[error("Multiple Stations available. Not possible to determine the desired station.")]
	MultipleStationsFound,
}

pub struct ProviderConfig {
	retry_count: isize,
	reqwest_client: Client,
}

impl ProviderConfig {
	fn new(retry_count: isize) -> Result<ProviderConfig, ()> {
		if retry_count < 0 {
			return Err(());
		}

		let client = Client::new();

		Ok(ProviderConfig {
			retry_count,
			reqwest_client: client,
		})
	}
}

pub trait DepartureProvider {
	/// Tries to find a station with the provided query. Returns all possible matches
	fn find_station(query: &str) -> Option<Vec<Station>>;

	/// Tries to add the station with the given id to the list of queried stations.
	/// If no station/multiple stations match the id, the method fails
	fn add_station(id: &str) -> Result<Station, StationAddError>;

	/// Updates the departures of all active stations
	fn update_departures();

	// TODO
	fn get_departures<Tz: TimeZone>(station_id: &str, timezone: Tz) -> DepartureList<Tz>;
}
