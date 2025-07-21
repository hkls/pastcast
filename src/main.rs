mod configs;
mod db;
mod handlers;
mod models;
mod repositories;
mod router;
mod services;

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
    let app_state = configs::state::AppState::new(db_pool);

    // Wrap the application state in an Arc for shared ownership
    let app_state = Arc::new(app_state);

    // Create the router with the application state
    let app = router::router_with_state(app_state);

    // Define the address to listen on, defaulting to
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));

    // Start the server and listen on port defined in the environment variable
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
