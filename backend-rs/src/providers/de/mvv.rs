//! Provider

use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
	providers::common::{station::Station, ProviderConfig},
	util::error::HttpError,
};

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

	pub async fn search_stations(
		&self,
		query: &str,
	) -> Result<Vec<MvvStation>, Box<dyn std::error::Error>> {
		let request_url = format!("https://www.mvv-muenchen.de/?eID=stopFinder&query={query}");

		let response = self
			.provider_config
			.reqwest_client
			.get(request_url)
			.send()
			.await?;

		if response.status() != StatusCode::OK {
			return Err(Box::new(HttpError {
				code: response.status().as_u16(),
				msg: "The mvv api server returned a status != 200".to_string(),
			}));
		}

		let response = response.json::<MvvStopFinderResponse>().await?;

		if !response.success {
			return Err(Box::new(HttpError {
				code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
				msg: "The mvv api server returned a message with success==false".to_string(),
			}));
		}

		let station_list = response
			.results
			.iter()
			.filter(|result| result.anyType == "stop")
			.map(|result| MvvStation {
				id: result.id.clone(),
				name: result.name.clone(),
			})
			.collect();

		Ok(station_list)
	}
}

#[derive(Debug, Serialize, Deserialize)]
struct MvvStopFinderResponse {
	success: bool,
	message: String,
	results: Vec<MvvStopFinderStop>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MvvStopFinderStop {
	usage: String,
	r#type: String,
	anyType: String,
	name: String,
	stateless: String,
	r#ref: MvvStopFinderRef,
	id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
struct MvvStopFinderRef {
	id: String,
	gid: String,
	omc: String,
	placeID: String,
	place: String,
	coords: String,
}

// struct MvvLine {
//   number: String,
//   symbol: String,
//   direction: String,
//   stateless: String,
//   name: String,
// }

#[derive(Debug)]
pub struct MvvStation {
	id: String,
	name: String,
}

impl From<MvvStation> for Station {
	fn from(value: MvvStation) -> Self {
		Station {
			id: value.id,
			name: value.name,
		}
	}
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
