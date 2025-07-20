use crate::config::config::Config;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;

/// Initializes a Postgres connection pool using the configuration from the environment.
pub async fn init_pg_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = Config::from_env()
        .map_err(|e| sqlx::Error::InvalidArgument(e.to_string()))?
        .database_url;

    PgPoolOptions::new()
        .max_connections(25)
        .idle_timeout(Some(Duration::from_secs(30)))
        .max_lifetime(Duration::from_secs(60 * 5)) // 5 minutes
        .connect(database_url.as_str())
        .await
}
