//! Handlers for the healthcheck related api endpoints

/// Handler for the ping endpoint. Returns an empty str
pub async fn ping() -> &'static str {
    ""
}
