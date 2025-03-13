use std::fmt::{Debug, Display, Pointer};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use de::mvg::MvgProvider;
use nu_ansi_term::Style;
use terminal_link::Link;
use themes::Theme;
use thiserror::Error;

pub mod de;
pub mod themes;

#[derive(Debug)]
pub struct MapCoordinate {
    pub latitude: f64,
    pub longitude: f64,
}

impl MapCoordinate {
    pub fn get_openstreetmap_org_link(&self) -> String {
        format!(
            "http://www.openstreetmap.org/?mlat={}&mlon={}&zoom=18&layers=M",
            self.latitude, self.longitude
        )
    }

    pub fn get_display_openstreetmap_org_link(&self) -> String {
        Link::new(&format!("{self}"), &self.get_openstreetmap_org_link()).to_string()
    }
}

impl Display for MapCoordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.6}, {:6})", self.latitude, self.longitude)
    }
}

#[derive(Debug)]
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
        let displayValue = match self {
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

        write!(f, "{displayValue}")
    }
}

#[derive(Debug)]
pub struct Line {
    pub label: String,
    pub r#type: TransportType,
    pub foreground_color: String,
    pub background_color: String,
    pub rail_replacement_bus_service: bool,
}

#[derive(Debug)]
pub struct Station {
    pub name: String,
    pub id: String,
    pub provider_id: &'static str,
    pub provider_internal_id: String,
    pub available_lines: Option<Vec<Line>>,
    pub available_transport_types: Option<Vec<TransportType>>,
    pub place: Option<String>,
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

        if let Some(ref transport_types) = self.available_transport_types {
            write!(
                f,
                "  available transport types: {}\n",
                transport_types
                    .iter()
                    .map(|tt| format!("{}", tt))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if let Some(ref lines) = self.available_lines {
            write!(
                f,
                "  available lines: {}\n",
                lines
                    .iter()
                    .map(|line| format!("{}", line.label))
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        writeln!(f, "  id: {}", Style::new().underline().paint(&self.id))?;
        writeln!(f, "  provider: {}", self.provider_id)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Departure {
    pub line: Line,
    pub destination: String,
    pub time_planned: DateTime<Utc>,
    pub time_live: Option<DateTime<Utc>>,
    pub occupancy: Option<u8>,
    pub cancelled: Option<bool>,
    pub rail_replacement_bus_service: Option<bool>,
    pub platform: Option<u8>,
    pub platform_changed: Option<bool>,
}

#[derive(Error, Debug)]
pub enum StationSearchResult {
    #[error("Failed to fetch data from the server")]
    FailedFetch(reqwest::Error),
    #[error("Unknown provider")]
    ProviderNotFound,
}

#[async_trait]
pub trait TransportProvider: Debug {
    async fn add_station(&mut self, name: &str);
    fn get_station(&self, id: &str) -> Option<&Station>;
    fn get_all_stations(&self) -> &Vec<Station>;
    async fn get_station_schedule(&self, id: &str) -> &Vec<Departure>;
    fn get_theme(&self) -> &Theme;
    async fn search_station(&self, query: &str) -> Result<Vec<Station>, StationSearchResult>;
}

pub fn get_new_provider_by_id(id: &str) -> Option<Box<dyn TransportProvider>> {
    return match id {
        "de/mvg" => Some(Box::new(MvgProvider::new())),
        _ => None,
    };
}
