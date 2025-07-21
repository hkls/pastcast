use crate::services::weather_service::WeatherService;
use sqlx::PgPool;

pub mod weather_service;

#[derive(Clone)]
pub struct Services {
    pub weather_service: WeatherService,
}

impl Services {
    pub fn new(pool: PgPool) -> Self {
        // Create a new WeatherService instance with the provided database connection pool.
        let weather_service = WeatherService::new(pool);

        Services { weather_service }
    }
}
