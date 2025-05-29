//! Create and attach the api endpoints to the router

use std::sync::Arc;

use axum::Router;

use super::app_state::AppState;
use super::error::handle_not_found;
use super::routes::attach_routes;

/// Create a new router with the required routes already attached
pub fn create_router(app_state: Arc<AppState>) -> Router {
    let router = Router::new();
    attach_fallbacks(attach_routes(router)).with_state(app_state)
}

fn attach_fallbacks(router: Router<Arc<AppState>>) -> Router<Arc<AppState>> {
    router.fallback(handle_not_found)
}
