use std::{collections::HashMap, time::Duration};

use async_trait::async_trait;
use futures::future;
use reqwest::Client;
use serde::Deserialize;
use tokio::time::sleep;
use tracing::warn;

use crate::{
    clients::transport::{
        Departure, Line, MapCoordinate, Occupancy, Station, StationsSearchError, TransportProvider,
        TransportType,
    },
    util::{
        get_list_of_pause_durations_from_normal_distribution,
        get_pause_duration_from_normal_distribution,
    },
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
            rail_replacement_bus_service: Some(value.sev),
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

#[derive(Deserialize, Debug)]
struct MvgDeparture {
    #[serde(rename = "plannedDepartureTime")]
    planned_departure_time: u64,
    realtime: bool,
    // realtime	true
    #[serde(rename = "delayInMinutes")]
    delay_in_minutes: i32,
    // delayInMinutes	0
    #[serde(rename = "realtimeDepartureTime")]
    realtime_departure_time: u64,
    // realtimeDepartureTime	1749153060000
    #[serde(rename = "transportType")]
    transport_type: String,
    label: String,
    destination: String,
    cancelled: bool,
    #[serde(rename = "sev")]
    rail_replacement_bus_service: bool,
    messages: Vec<String>,
    infos: Vec<String>,
    #[serde(deserialize_with = "crate::clients::transport::deserialize_occupancy")]
    occupancy: Option<Occupancy>,
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
pub struct MvgProvider {
    stations: Box<HashMap<String, Station>>,
    departures: Box<HashMap<String, Vec<Departure>>>,
    departures_precomputed_json: Box<HashMap<String, String>>,
    client: Box<Client>,
}

impl MvgProvider {
    #[must_use]
    pub fn new() -> Self {
        MvgProvider {
            stations: Box::new(HashMap::new()),
            client: Box::new(Client::new()),
            departures: Box::new(HashMap::new()),
            departures_precomputed_json: Box::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TransportProvider for MvgProvider {
    async fn add_station(&mut self, name: &str) -> Result<(), StationsSearchError> {
        let api_station_result = self.search_stations(name, false).await?;
        if api_station_result.len() > 1 {
            warn!("More than one station found");
        }
        if let Some(station) = api_station_result.into_iter().next() {
            self.stations.insert(name.to_string(), station);
            return Result::Ok(());
        }

        return Err(StationsSearchError::NoMatchFound);
    }
    fn get_station(&self, id: &str) -> Option<&Station> {
        self.stations.get(id)
    }
    fn get_all_stations(&self) -> Vec<&Station> {
        self.stations.values().collect()
    }

    fn get_station_schedule(&self, id: &str) -> Option<&Vec<Departure>> {
        self.departures.get(id)
    }

    fn get_complete_schedule_json(&self, id: &str) -> Option<&String> {
        self.departures_precomputed_json.get(id)
    }

    async fn update_departures(
        &mut self,
        min_pause: Duration,
        max_pause: Duration,
    ) -> anyhow::Result<()> {
        let keys = self.stations.keys();

        // Precompute pauses to avoid async issues with putting the thread rng on another thread,
        // which defeats the purpose of using a thread rng.
        let pauses =
            get_list_of_pause_durations_from_normal_distribution(min_pause, max_pause, keys.len());

        for (id, pause) in keys.zip(pauses) {
            // Pause between requests to avoid rate limiting.
            // Use a normal distribution to avoid being too regular.
            sleep(pause).await;
            update_departures_for_station(&mut self.client, &mut self.departures, &id).await?;
        }

        Ok(())
    }

    async fn search_stations(
        &self,
        query: &str,
        detailed: bool,
    ) -> Result<Vec<Station>, StationsSearchError> {
        let url = format!(
            "https://www.mvg.de/api/bgw-pt/v3/locations?query={}&locationTypes=STATION",
            query
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(StationsSearchError::from)?;

        let mvg_locations: Vec<MvgLocation> =
            response.json().await.map_err(StationsSearchError::from)?;

        let stations: Vec<Station> =
            future::try_join_all(mvg_locations.iter().map(async |location| {
                let mut station = Station::from(location);

                if !detailed {
                    return Ok(station);
                }

                let lines = location.fetch_lines(&self.client).await?;

                station.available_lines = Some(lines);
                Ok(station)
            }))
            .await
            .map_err(|err| StationsSearchError::FailedFetch(err))?;

        Ok(stations)
    }
}

async fn update_departures_for_station(
    client: &mut Client,
    departures: &mut HashMap<String, Vec<Departure>>,
    id: &str,
) -> anyhow::Result<()> {
    {
        let url = format!(
            "https://www.mvg.de/api/bgw-pt/v3/departures?globalId={}&limit=100",
            id
        );

        println!("{url}");

        let response = client.get(&url).send().await?;

        // -------------
        // WRONG???????
        // -------------

        // let locations: Vec<MvgLocation> = response.json().await?;

        // let stations: Vec<_> = locations
        //     .iter()
        //     .map(|location| {
        //         let mut station = Station::from(location);

        //         station
        //     })
        //     .collect();

        todo!("Implement writeback and stuff");

        Ok(())
    }
}
