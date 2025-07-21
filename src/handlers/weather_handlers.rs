use crate::configs::state::AppState;
use axum::extract::State;
use sqlx::postgres::types::PgPoint;
use std::sync::Arc;

pub async fn home_handler(State(state): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    match state
        .services
        .weather_service
        .get_weather_data(PgPoint { x: 0.0, y: 0.0 })
        .await
    {
        Ok(_) => {
            // Access values from the shared config
            format!("Welcome to {} v{}", 1, 2)
        }
        Err(err) => {
            eprintln!("Error fetching weather data: {:?}", err);
            axum::response::Response::builder()
                .status(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to fetch weather data".into())
                .unwrap()
        }
    }
}
