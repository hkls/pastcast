mod config;
mod db;
mod handlers;
mod router;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize the PostgreSQL connection pool
    let db_pool = db::init_pg_pool()
        .await
        .expect("Failed to initialize database connection pool");

    // Create the application state with the database connection pool
    let app_state = config::app_state::AppState::new(db_pool);

    // Wrap the application state in an Arc for shared ownership
    let app_state = Arc::new(app_state);

    // Create the router with the application state
    let app = router::router_with_state(app_state);

    // Start the server and listen on port defined in the environment variable or default to 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
