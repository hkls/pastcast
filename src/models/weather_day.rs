use chrono::{DateTime, Utc};
use sqlx::FromRow;
use sqlx::postgres::types::PgPoint;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct WeatherDay {
    pub id: i64,
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: i64,

    pub date: DateTime<Utc>,

    pub location: String,
    pub region: String,
    pub country: String,

    // PostgreSQL `point` should be mapped as (f64, f64) or use a custom type.
    pub latlon: PgPoint,

    pub unit: String,

    pub astro_sunrise: DateTime<Utc>,
    pub astro_sunset: DateTime<Utc>,
    pub astro_moonrise: DateTime<Utc>,
    pub astro_moonset: DateTime<Utc>,
    pub astro_moon_phase: String,
    pub astro_moon_illumination: f64,

    pub mintemp: f64,
    pub maxtemp: f64,
    pub avgtemp: f64,
    pub totalsnow: f64,
    pub sunhour: f64,
    pub uv_index: f64,
}
