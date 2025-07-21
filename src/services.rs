use crate::configs::weather_stack::WeatherStackHttpClient;
use crate::services::weather_service::WeatherService;
use sqlx::PgPool;

pub mod weather_service;

#[derive(Clone)]
pub struct Services {
    pub weather_service: WeatherService,
    pub weather_stack: WeatherStackHttpClient,
}

impl Services {
    pub fn new(pool: PgPool) -> Self {
        // Create a new WeatherService instance with the provided database connection pool.
        let weather_service = WeatherService::new(pool);
        let weather_stack = WeatherStackHttpClient::new();

        Services {
            weather_service,
            weather_stack,
        }
    }
}
