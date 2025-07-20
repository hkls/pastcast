use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WeatherHourly {
    pub id: i64,
    pub uuid: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,

    pub weather_days_id: i64,

    pub hour: String,
    pub time: DateTime<Utc>,
    pub temperature: f64,
    pub wind_speed: f64,
    pub wind_degree: f64,
    pub wind_dir: String,
    pub weather_code: i32,
    pub weather_descriptions: Option<Value>, // jsonb -> serde_json::Value

    pub precip: f64,
    pub humidity: f64,
    pub visibility: f64,
    pub pressure: f64,
    pub cloudcover: f64,
    pub heatindex: f64,
    pub dewpoint: f64,
    pub windchill: f64,
    pub windgust: f64,
    pub feelslike: f64,

    pub chanceofrain: f64,
    pub chanceofremdry: f64,
    pub chanceofwindy: f64,
    pub chanceofovercast: f64,
    pub chanceofsunshine: f64,
    pub chanceoffrost: f64,
    pub chanceofhightemp: f64,
    pub chanceoffog: f64,
    pub chanceofsnow: f64,
    pub chanceofthunder: f64,

    pub uv_index: f64,
}
