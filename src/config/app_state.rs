#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
}

pub struct AppConfig {
    server_port: u16,
}

/// This struct holds the application state, including the database connection pool.
impl AppState {
    pub fn new(db: sqlx::PgPool) -> Self {
        AppState { db }
    }
}
