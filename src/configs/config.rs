use std::env;

pub struct EnvConfig {
    pub db_spec: DBSpec,
    pub weatherstack_spec: WeatherStackSpec,
}

pub struct DBSpec {
    pub database_url: String,
}

pub struct WeatherStackSpec {
    pub api_url: String,
    pub api_key: String,
}

impl EnvConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        let db_spec = DBSpec {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://user:password@localhost:5432/pastcast".to_string()),
        };

        let weatherstack_spec = WeatherStackSpec {
            api_key: env::var("WEATHER_STACK_API_KEY").unwrap_or_else(|_| "".to_string()),
            api_url: env::var("WEATHER_STACK_API_URL")
                .unwrap_or_else(|_| "https://api.weatherstack.com".to_string()),
        };

        Ok(Self {
            db_spec,
            weatherstack_spec,
        })
    }
}
