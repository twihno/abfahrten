use std::str::FromStr;

use envconfig::Envconfig;
use thiserror::Error;

#[derive(Envconfig, Clone, Debug)]
pub struct SingleStationRequest {
    pub country: String,
    pub provider: String,
    pub station: String,
    pub theme: Option<String>,
}

#[derive(Debug, Clone, Error)]
pub enum ParseSingleStationRequestError {
    #[error("Request is empty")]
    EmptyRequest,
    #[error("The given parameters don't match the required amount")]
    NotEnoughArguments,
}

impl FromStr for SingleStationRequest {
    type Err = ParseSingleStationRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_requests = s.split(":");

        let country = split_requests
            .next()
            .ok_or(ParseSingleStationRequestError::EmptyRequest)?
            .to_string();
        let provider = split_requests
            .next()
            .ok_or(ParseSingleStationRequestError::NotEnoughArguments)?
            .to_string();
        let station = split_requests
            .next()
            .ok_or(ParseSingleStationRequestError::NotEnoughArguments)?
            .to_string();

        let theme = match split_requests.next() {
            Some(value) => Some(value.to_string()),
            None => None,
        };

        let request = SingleStationRequest {
            country,
            provider,
            station,
            theme,
        };

        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct StationRequests {
    pub requests: Vec<SingleStationRequest>,
}

impl FromStr for StationRequests {
    type Err = ParseSingleStationRequestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let requests: Vec<SingleStationRequest> = s
            .split(";")
            .map(|s| SingleStationRequest::from_str(s))
            .collect::<Result<_, _>>()?;

        Ok(StationRequests { requests })
    }
}
#[derive(Envconfig, Clone, Debug)]
pub struct AppConfig {
    #[envconfig(from = "STATIONS")]
    pub station_requests: StationRequests,

    #[envconfig(nested)]
    pub server_config: ServerConfig,
}

#[derive(Envconfig, Clone, Debug)]
pub struct ServerConfig {
    #[envconfig(from = "PORT", default = "8000")]
    pub port: u16,

    #[envconfig(from = "BASE_URL", default = "/")]
    pub base_url: String,

    #[envconfig(from = "ONLY_LOCALHOST", default = "false")]
    pub only_localhost: bool,
}
