//! Provider

use crate::providers::common::ProviderConfig;

pub struct MvvProvider<'a> {
	monitored_stations: Vec<String>,
	provider_config: &'a ProviderConfig,
}

impl<'a> MvvProvider<'a> {
	pub fn new(provider_config: &ProviderConfig) -> MvvProvider {
		MvvProvider {
			monitored_stations: vec![],
			provider_config,
		}
	}

	fn search_stations(&self, query: &str) -> Result<Vec<MvvStation>, Box<dyn std::error::Error>> {
		let mut found_stations = vec![];

		self.Ok(found_stations)
	}
}

// struct MvvLine {
//   number: String,
//   symbol: String,
//   direction: String,
//   stateless: String,
//   name: String,
// }

pub struct MvvStation {
	id: String,
	name: String,
}

// struct MvvNotification {
//   text: String,
//   link: String,
//   r#type: String,
// }

// struct MvvDeparture {
//   line: line,
//   direction: String,
//   station: Station,
//   track: String,
//   departureDate: String,
//   departurePlanned: String,
//   departureLive: String,
//   inTime: bool,
//   notifications: notification[]
// }

// struct MvvApiResponse {
//   error: String,
//   departures: mvvDeparture[]
// }
