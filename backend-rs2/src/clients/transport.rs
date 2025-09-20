use std::{
    fmt::{Debug, Display},
    time::Duration,
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use de::mvg::MvgProvider;
use nu_ansi_term::Style;
use serde::Deserialize;
use terminal_link::Link;
use thiserror::Error;

pub mod de;
pub mod themes;

#[derive(Debug, Clone)]
/// Description of the occupancy level of a vehicle.
pub enum Occupancy {
    Low,
    Middle,
    High,
}

impl TryFrom<&str> for Occupancy {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "low" => Ok(Occupancy::Low),
            "middle" => Ok(Occupancy::Middle),
            "high" => Ok(Occupancy::High),
            "0" => Ok(Occupancy::Low),
            "1" => Ok(Occupancy::Middle),
            "2" => Ok(Occupancy::High),
            _ => Err(()),
        }
    }
}

pub fn deserialize_occupancy<'de, D>(deserializer: D) -> Result<Option<Occupancy>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) => Ok(Occupancy::try_from(s.as_str()).ok()),
        None => Ok(None),
    }
}

#[derive(Debug, Clone)]
/// A simple representation of a geographic coordinate.
pub struct MapCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl MapCoordinate {
    /// Returns a link to openstreetmap.org with a marker at the coordinate.
    pub fn get_openstreetmap_org_link(&self) -> String {
        format!(
            "http://www.openstreetmap.org/?mlat={}&mlon={}&zoom=18&layers=M",
            self.latitude, self.longitude
        )
    }

    /// Returns a clickable console link (ANSI escaped) to openstreetmap.org with a marker at the coordinate.
    pub fn get_display_openstreetmap_org_link(&self) -> String {
        Link::new(&format!("{self}"), &self.get_openstreetmap_org_link()).to_string()
    }
}

impl Display for MapCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.6}, {:6})", self.latitude, self.longitude)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// A representation of a type of transport, e.g. bus or train.
pub enum TransportType {
    Tram,
    Train,
    UndergroundTrain,
    SuburbanTrain,
    RegionalTrain,
    LongDistanceTrain,
    HighspeedTrain,
    Bus,
    BusRapidTransit,
    LongDistanceBus,
    Ferry,
}

impl Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_value = match self {
            TransportType::Tram => "tram",
            TransportType::Train => "train",
            TransportType::UndergroundTrain => "underground train",
            TransportType::SuburbanTrain => "suburban train",
            TransportType::RegionalTrain => "regional train",
            TransportType::LongDistanceTrain => "long distance train",
            TransportType::HighspeedTrain => "highspeed train",
            TransportType::Bus => "bus",
            TransportType::BusRapidTransit => "rapid transit bus",
            TransportType::LongDistanceBus => "long distance bus",
            TransportType::Ferry => "ferry",
        };

        write!(f, "{display_value}")
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    /// The label of the line, e.g. "U1" or "S7".
    pub label: String,
    /// The type of transport, e.g. bus or train.
    /// If the type is unknown, this is `None`.
    pub r#type: Option<TransportType>,
    /// Whether this line is a rail replacement bus service.
    pub rail_replacement_bus_service: Option<bool>,
}

#[derive(Debug, Clone)]
/// A representation of a station/stop.
pub struct Station {
    /// The name of the station, e.g. "Marienplatz".
    pub name: String,
    /// The unique id of the station.
    /// This is usually assigned by the provider and provider-specific.
    pub id: String,
    /// The id of the provider that provides this station.
    pub provider_id: &'static str,
    /// The internal id of the station within the provider.
    pub provider_internal_id: String,
    /// The lines that are available at this station.
    /// If the provider does not provide this information, this is `None`.
    pub available_lines: Option<Vec<Line>>,
    /// The types of transport that are available at this station.
    /// If the provider does not provide this information, this is `None`.
    pub available_transport_types: Option<Vec<TransportType>>,
    /// A description of the place where the station is located, e.g. the city or district.
    /// If the provider does not provide this information, this is `None`.
    pub place: Option<String>,
    /// The geographic location of the station.
    /// If the provider does not provide this information, this is `None`.
    pub location: Option<MapCoordinate>,
}

impl Display for Station {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Style::new().bold().underline().paint(&self.name))?;

        if let Some(ref place) = self.place {
            if let Some(ref location) = self.location {
                write!(
                    f,
                    " ({place} {})",
                    location.get_display_openstreetmap_org_link()
                )?;
            } else {
                write!(f, "  ({place})")?;
            }
        } else {
            if let Some(ref location) = self.location {
                write!(f, "  {}", location.get_display_openstreetmap_org_link())?;
            }
        }

        write!(f, "\n")?;

        writeln!(f, "  id:\n    {}", Style::new().underline().paint(&self.id))?;
        writeln!(f, "  provider:\n    {}", self.provider_id)?;

        if let Some(ref transport_types) = self.available_transport_types {
            write!(
                f,
                "  available transport types:\n{}\n",
                transport_types
                    .iter()
                    .map(|tt| format!("    - {}", tt))
                    .collect::<Vec<_>>()
                    .join("\n")
            )?;
        }

        if let Some(ref lines) = self.available_lines {
            write!(
                f,
                "  available lines:\n{}\n",
                lines
                    .iter()
                    .map(|line| format!("    - {}", line.label))
                    .collect::<Vec<_>>()
                    .join("\n")
            )?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Departure {
    /// The short label of the vehicle, e.g. "U1" or "S7".
    pub label: String,
    /// The destination of the vehicle, e.g. "Olympia-Einkaufszentrum".
    pub destination: String,
    /// The planned departure time.
    pub time_planned: DateTime<Utc>,
    /// The current live departure time.
    /// If live data is not available, this is `None`.
    /// Also, if `time_live` is unequal to `time_planned`, the departure is delayed or early.
    pub time_live: Option<DateTime<Utc>>,
    /// The occupancy level of the vehicle, if available.
    /// If the occupancy level is not available, this is `None`.
    pub occupancy: Option<Occupancy>,
    /// Is the departure cancelled? If unknown, this is `None`.
    pub cancelled: Option<bool>,
    /// Is this departure part of a rail replacement bus service? If unknown, this is `None`.
    pub rail_replacement_bus_service: Option<bool>,
    /// Platform name/number, if available.
    pub platform: Option<String>,
    /// Whether the platform has changed from the originally planned one. If unknown, this is `None`.
    pub platform_changed: Option<bool>,
    /// Additional messages related to this departure, e.g. construction information or high traffic notices.
    pub messages: Vec<Message>,
}

#[derive(Error, Debug)]
pub enum StationsSearchError {
    #[error("Failed to fetch data from the server")]
    FailedFetch(anyhow::Error),
    #[error("No matching station found on server")]
    NoMatchFound,
}

impl From<reqwest::Error> for StationsSearchError {
    fn from(value: reqwest::Error) -> Self {
        StationsSearchError::FailedFetch(value.into())
    }
}

/// A message type related to a departure, e.g. a delay or cancellation.
#[derive(Debug)]
pub enum MessageType {
    Info,
    Warning,
    Construction,
    Urgent,
}

/// A message related to a departure, e.g. a delay or cancellation.
#[derive(Debug)]
pub struct Message {
    /// The type of the message, e.g. info or warning.
    /// If unknown, this is `Info`.
    r#type: MessageType,
    /// The unique id of the message.
    /// If the provider does not provide this,
    /// this has to be set to a unique value, e.g. the message hash.
    id: String,
    /// The content of the message.
    content: String,
}

#[async_trait]
pub trait TransportProvider: Debug {
    /// Try to add a station by name.
    /// If multiple stations match the name, the first one is chosen.
    /// The station is then stored internally and can be accessed via `get_station` or `get_all_stations`.
    /// The corresponding departures are then fetched on a call to `update_departures`.
    ///
    /// # Errors
    /// - Returns [`StationsSearchError::FailedFetch`] if the station search request fails.
    /// - Returns [`StationsSearchError::NoMatchFound`] if no station matches the given name.
    async fn add_station(&mut self, name: &str) -> Result<(), StationsSearchError>;

    /// Try to get a stored station by its internal id.
    fn get_station(&self, id: &str) -> Option<&Station>;

    /// Get all stored stations.
    fn get_all_stations(&self) -> Vec<&Station>;

    /// Update the departures for all stored stations.
    /// This fetches the latest departure data from the provider's API and updates the internal state.
    /// The `min_pause` and `max_pause` parameters can be used to introduce a delay between requests to avoid rate limiting.
    /// If the provider has no rate limiting, these parameters can be set to zero.
    ///
    ///
    /// # Errors
    /// - Returns an error if the departure fetch request fails.
    async fn update_departures(
        &mut self,
        min_pause: Duration,
        max_pause: Duration,
    ) -> anyhow::Result<()>;

    fn get_station_schedule(&self, id: &str) -> Option<&Vec<Departure>>;

    /// Get the precomputed JSON string for the departure schedule of a station by its internal id.
    /// This is used to serve the departure schedule via the API.
    /// The JSON string is precomputed during `update_departures` to avoid computing it on each request.
    /// If no station with the given id is found, `None` is returned.
    fn get_complete_schedule_json(&self, id: &str) -> Option<&String>;

    /// Search for stations matching the given query.
    /// This does not modify the internal state of the provider and is only used for searching.
    ///
    /// # Parameters
    /// - `query`: The search query string.
    /// - `detailed`: If true, the returned stations will include additional details such as available lines and transport types.
    ///
    /// # Errors
    /// - Returns [`StationsSearchError::FailedFetch`] if the station search request fails.
    /// - Returns [`StationsSearchError::NoMatchFound`] if no station matches the given query.
    async fn search_stations(
        &self,
        query: &str,
        detailed: bool,
    ) -> Result<Vec<Station>, StationsSearchError>;
}

/// Create a new transport provider instance based on the given provider id.
/// Returns `None` if the provider id is not recognized/not implemented.
pub fn get_new_provider_by_id(id: &str) -> Option<Box<dyn TransportProvider>> {
    return match id {
        "de.mvg" => Some(Box::new(MvgProvider::new())),
        _ => None,
    };
}

pub fn get_all_available_providers() -> Vec<(&'static str, &'static str)> {
    vec![("de.mvg", "Münchner Verkehrsgesellschaft")]
}
