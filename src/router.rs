use crate::configs::state::AppState;
use crate::handlers::weather_handlers::home_handler;
use axum::{Router, routing::get};
use std::sync::Arc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn router_with_state(state: Arc<AppState>) -> Router {
    // Set up logging with environment-based filter
    let layer = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    Router::new()
        .route("/", get(home_handler))
        .with_state(state)
        .layer(layer)
}
