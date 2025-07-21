use crate::models::weather_day::WeatherDay;
use crate::repositories::weather_repositories::{Repository, WeatherRepository};
use sqlx::PgPool;
use sqlx::postgres::types::PgPoint;
use std::sync::Arc;

#[derive(Clone)]
pub struct WeatherService {
    repo: Arc<Repository>,
}

impl WeatherService {
    pub fn new(pool: PgPool) -> Self {
        // Let the Service take care of constructing itself.  Hide the complexity from the main.rs
        // i.e., other services might be injected here as well, e.g., a cache service or a logging service.
        let repo = Arc::new(Repository::new(pool));

        WeatherService { repo }
    }

    /// Fetch weather data for a specific city using its geographical point.
    /// This method is used to retrieve weather data for a specific location.
    pub async fn get_weather_data(
        &self,
        pg_point: PgPoint,
    ) -> Result<Vec<WeatherDay>, sqlx::Error> {
        self.repo.get_weather_data(pg_point).await
    }
}
