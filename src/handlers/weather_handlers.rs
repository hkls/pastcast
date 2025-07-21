use crate::configs::state::AppState;
use axum::extract::State;
use sqlx::postgres::types::PgPoint;
use std::sync::Arc;

pub async fn home_handler(State(state): State<Arc<AppState>>) -> String {
    state
        .services
        .weather_service
        .get_weather_data(PgPoint { x: 0.0, y: 0.0 })
        .await
        .unwrap();

    // Access values from the shared config
    format!("Welcome to {} v{}", 1, 2)
}
