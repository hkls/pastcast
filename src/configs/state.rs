use crate::services::Services;

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}

/// This struct holds the application state, including the database connection pool.
impl AppState {
    pub fn new(db: sqlx::PgPool) -> Self {
        let services = Services::new(db);

        AppState { services }
    }
}
