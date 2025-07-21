use crate::config::app_state::AppState;
use axum::extract::State;
use std::sync::Arc;

pub async fn home_handler(State(state): State<Arc<AppState>>) -> String {
    state.db.begin().await.expect("Failed to begin transaction");

    // Access values from the shared config
    format!("Welcome to {} v{}", 1, 2)
}
