use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;

use crate::clients::transport::{
    Departure, Line, MapCoordinate, Station, StationSearchResult, TransportProvider, TransportType,
    themes::Theme,
};

#[derive(Deserialize, Debug)]
struct MvgLine {
    label: String,
    #[serde(rename = "transportType")]
    transport_type: String,
    sev: bool,
}

impl From<&MvgLine> for Line {
    fn from(value: &MvgLine) -> Self {
        Line {
            label: value.label.clone(),
            r#type: convert_mvg_transport_type(&value.transport_type),
            rail_replacement_bus_service: value.sev,
            background_color: String::from("#"),
            foreground_color: String::from("#"),
        }
    }
}

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

#[derive(Debug)]
struct MvgLineFetchJob<'a> {
    location: &'a mut MvgLocation,
    // job: Future<()>,
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

impl MvgLocation {
    async fn fetch_lines(&self, client: &Client) -> anyhow::Result<Vec<Line>> {
        let url = format!("https://www.mvg.de/api/bgw-pt/v3/lines/{}", self.global_id);

        let response = client.get(url).send().await?;

        let lines: Vec<MvgLine> = response.json().await?;

        let lines: Vec<Line> = lines.iter().map(|line| Line::from(line)).collect();

        Ok(lines)
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
        panic!("Not implemented!")
    }
    fn get_station(&self, id: &str) -> Option<&Station> {
        panic!("Not implemented!")
    }
    fn get_all_stations(&self) -> &Vec<Station> {
        panic!("Not implemented!")
    }
    async fn get_station_schedule(&self, id: &str) -> &Vec<Departure> {
        panic!("Not implemented!")
    }
    fn get_theme(&self) -> &Theme {
        panic!("Not implemented!")
    }
    async fn search_station(
        &self,
        query: &str,
        detailed: bool,
    ) -> Result<Vec<Station>, StationSearchResult> {
        let client = Client::new();

        let url = format!(
            "https://www.mvg.de/api/bgw-pt/v3/locations?query={}&locationTypes=STATION",
            query
        );

        println!("{url}");

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|err| StationSearchResult::FailedFetch(err))?;
        let mut locations: Vec<MvgLocation> = response
            .json()
            .await
            .map_err(|err| StationSearchResult::FailedFetch(err))?;

        let stations: Vec<_> = locations
            .iter()
            .map(|location| {
                let mut station = Station::from(location);
                // if detailed {
                //     location
                //         .fetch_lines(&client)
                //         .then(async |lines| match lines {
                //             Ok(value) => station.available_lines = Some(value),
                //             Err(err) => {
                //                 error!("{}", err);
                //             }
                //         });
                // }

                station
            })
            .collect();

        // let fetch_jobs: Vec<_> = Vec::with_capacity(locations.len());

        // if detailed {
        //     let a = locations
        //         .iter_mut()
        //         .map(async |location| match location.fetch_lines(&client).await {
        //             Ok(value) => location.available_lines = Some(value),
        //             Err(err) => {
        //                 error!("{}", err);
        //             }
        //         })
        //         .collect();

        //     // for station in stations {
        //     //     fetch_jobs.push(async || match station.fetch_lines(&client) {});
        //     // }
        // }

        // let locations = futures::future::join_all(fetch_jobs).await;

        Ok(stations)
    }
}
