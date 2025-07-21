use crate::configs::state::AppState;
use crate::handlers::weather_handlers::home_handler;
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn router_with_state(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(home_handler))
        .with_state(state)
}
