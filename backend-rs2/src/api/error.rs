//! Handlers for errors in api calls

use axum::{
    extract::{Host, OriginalUri},
    http::StatusCode,
    response::Html,
};
use tracing::error;

macro_rules! asset {
    ($x: expr) => {
        include_str!(concat!("../../assets/", $x))
    };
}

/// Default type for the error pages
pub type ErrorPage = (StatusCode, Html<&'static str>);

/// Handler for the default fallback: 404 - Page not found
pub async fn handle_not_found(Host(host): Host, OriginalUri(uri): OriginalUri) -> ErrorPage {
    error!("404 - Page not found: \"{}{:?}\"", host, uri);
    get_404_response()
}

/// Return the 400 error HTML page
pub fn get_400_response() -> ErrorPage {
    (StatusCode::BAD_REQUEST, Html(asset!("400.html")))
}

/// Return the 404 error HTML page
pub fn get_404_response() -> ErrorPage {
    (StatusCode::NOT_FOUND, Html(asset!("404.html")))
}

/// Return the 409 error HTML page
pub fn get_409_response() -> ErrorPage {
    (StatusCode::CONFLICT, Html(asset!("409.html")))
}

/// Return the 418 error HTML page
pub fn get_418_response() -> ErrorPage {
    (StatusCode::IM_A_TEAPOT, Html(asset!("418.html")))
}

/// Return the 500 error HTML page
pub fn get_500_response() -> ErrorPage {
    (StatusCode::INTERNAL_SERVER_ERROR, Html(asset!("500.html")))
}

/// Replace an error with the default 400 page.
///
/// Accepts anything that implements `std::fmt::Debug`,
/// logs the value as an error and returns the default 400 page
pub fn replace_with_400_page<E: std::fmt::Debug>(err: E) -> ErrorPage {
    error!("{:?}", err);
    get_400_response()
}

/// Replace an error with the default 500 page.
///
/// Accepts anything that implements `std::fmt::Debug`,
/// logs the value as an error and returns the default 500 page
pub fn replace_with_500_page<E: std::fmt::Debug>(err: E) -> ErrorPage {
    error!("{:?}", err);
    get_500_response()
}
