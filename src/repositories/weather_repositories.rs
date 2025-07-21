use crate::models::weather_day::WeatherDay;
use sqlx::PgPool;
use sqlx::postgres::types::PgPoint;

pub trait WeatherRepository: Send + Sync {
    async fn get_weather_data(&self, pg_point: PgPoint) -> Result<Vec<WeatherDay>, sqlx::Error>;
}

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl WeatherRepository for Repository {
    // Get weather data for a specific city using its geographical point.  Historically, this might
    // be used to fetch weather data for a specific location.
    async fn get_weather_data(&self, pg_point: PgPoint) -> Result<Vec<WeatherDay>, sqlx::Error> {
        let vec = sqlx::query_as::<_, WeatherDay>("select * from weather_days where id = $1")
            .bind(pg_point.x)
            .fetch_all(&self.pool)
            .await?;

        Ok(vec)
    }
}
