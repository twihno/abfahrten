use async_trait::async_trait;
use serde::Deserialize;

use crate::clients::transport::{
    Departure, MapCoordinate, Station, StationSearchResult, TransportProvider, TransportType,
    themes::Theme,
};

#[derive(Deserialize, Debug)]
struct MvgLocation {
    latitude: f64,
    longitude: f64,
    place: String,
    name: String,
    #[serde(rename = "globalId")]
    global_id: String,
    #[serde(rename = "transportTypes")]
    transport_types: Vec<String>,
}

fn convert_mvg_transport_type(transport_type: &str) -> Option<TransportType> {
    return match transport_type {
        "UBAHN" => Some(TransportType::UndergroundTrain),
        "SBAHN" => Some(TransportType::SuburbanTrain),
        "BUS" => Some(TransportType::Bus),
        "TRAM" => Some(TransportType::Tram),
        "BAHN" => Some(TransportType::Train),
        "SCHIFF" => Some(TransportType::Ferry),
        _ => None,
    };
}

impl From<&MvgLocation> for Station {
    fn from(value: &MvgLocation) -> Self {
        Station {
            name: value.name.clone(),
            id: format!("de/mvg::{}", value.global_id),
            provider_internal_id: value.global_id.clone(),
            provider_id: "de/mvg",
            available_lines: None,
            available_transport_types: Some(
                value
                    .transport_types
                    .iter()
                    .filter_map(|arg0: &String| convert_mvg_transport_type(&arg0))
                    .collect(),
            ),
            place: Some(value.place.clone()),
            location: Some(MapCoordinate {
                latitude: value.latitude,
                longitude: value.longitude,
            }),
        }
    }
}

#[derive(Debug)]
pub struct MvgProvider {}

impl MvgProvider {
    #[must_use]
    pub fn new() -> Self {
        MvgProvider {}
    }
}

#[async_trait]
impl TransportProvider for MvgProvider {
    async fn add_station(&mut self, name: &str) {
        panic!("AAA")
    }
    fn get_station(&self, id: &str) -> Option<&Station> {
        panic!("AAA")
    }
    fn get_all_stations(&self) -> &Vec<Station> {
        panic!("AAA")
    }
    async fn get_station_schedule(&self, id: &str) -> &Vec<Departure> {
        panic!("AAA")
    }
    fn get_theme(&self) -> &Theme {
        panic!("AAA")
    }
    async fn search_station(&self, query: &str) -> Result<Vec<Station>, StationSearchResult> {
        let url = format!(
            "https://www.mvg.de/api/bgw-pt/v3/locations?query={}&locationTypes=STATION",
            query
        );

        println!("{url}");

        let response = reqwest::get(&url)
            .await
            .map_err(|err| StationSearchResult::FailedFetch(err))?;
        let locations: Vec<MvgLocation> = response
            .json()
            .await
            .map_err(|err| StationSearchResult::FailedFetch(err))?;

        Ok(locations
            .iter()
            .map(|location| Station::from(location))
            .collect())
    }
}
