use crate::configs::state::AppState;
use axum::extract::State;
use sqlx::postgres::types::PgPoint;
use std::sync::Arc;

pub async fn home_handler(State(state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    match state
        .services
        .weather_service
        .get_weather_data(PgPoint {
            x: -33.9249,
            y: 18.4241,
        })
        .await
    {
        Ok(res) => {
            // Access values from the shared config
            format!("Welcome to {}", res.first().unwrap().country)
        }
        Err(err) => {
            format!("Failed to fetch weather data: {}", err)
        }
    }
}
