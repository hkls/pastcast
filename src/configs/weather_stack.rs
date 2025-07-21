use crate::models::weather_day::WeatherDay;
use reqwest::{Client, Error};
use sqlx::postgres::types::PgPoint;

#[derive(Clone)]
pub struct WeatherStackHttpClient {
    pub client: Client,
    base_url: String,
    api_key: String,
}

impl WeatherStackHttpClient {
    pub fn new() -> Self {

        // Configure a connection-pooled reqwest client
        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .pool_max_idle_per_host(8)
            .build()
            .unwrap();

        let (base_url, api_key) = match std::env::var("WEATHER_STACK_API_KEY") {
            Ok(key) => ("http://api.weatherstack.com".to_string(), key),
            Err(_) => panic!("WEATHER_STACK_API_KEY environment variable not set"),
        };

        WeatherStackHttpClient { client, base_url, api_key}
    }
}

pub trait WeatherStackClient {
    async fn get_current_weather_by_coords(&self, latlon: PgPoint) -> Result<WeatherDay, reqwest::Error>;
}

impl WeatherStackClient for WeatherStackHttpClient {
    async fn get_current_weather_by_coords(&self, latlon: PgPoint) -> Result<WeatherDay, Error> {
        let url = format!(
            "http://api.weatherstack.com/current?access_key={}&query={},{}",
            self.api_key, latlon.x, latlon.y
        );

        let res = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json::<WeatherDay>()
            .await?;

        Ok(res)
    }
}