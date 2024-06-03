pub mod departure;
pub mod station;

use chrono::TimeZone;
use reqwest::Client;

use crate::util::exit::exit_critical;

use self::{
	departure::DepartureList,
	station::{Station, StationAddError},
};

use super::de::mvv::MvvProvider;

pub struct ProviderConfig {
	pub retry_count: isize,
	pub reqwest_client: Client,
}

impl ProviderConfig {
	fn new(retry_count: isize) -> Result<ProviderConfig, String> {
		if retry_count < 0 {
			return Err(format!("Invalid retry_count: {retry_count} < 0"));
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

pub async fn ruuun() {
	let conf = match ProviderConfig::new(3) {
		Err(err) => exit_critical(&err, true),
		Ok(val) => val,
	};

	let mvv_provider = MvvProvider::new(&conf);

	match mvv_provider.search_stations("Marienplatz").await {
		Ok(val) => println!("{:?}", val),
		Err(err) => println!("ERROR: {err}"),
	}
}
