//! Functionality related to the creation and
//! attachment of api routes

use std::sync::Arc;

use axum::{Router, routing::get};
use healthcheck::ping;

use super::app_state::AppState;

pub mod healthcheck;

/// Attach routes to the provided router.
/// Creates a new router and returns the modified version
pub fn attach_routes(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    // TODO: Add base route handling
    router.route("/ping", get(ping))
}
