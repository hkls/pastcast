use std::env;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .ok()
                .unwrap_or("postgres://user:password@localhost:5432/pastcast".to_string()),
        })
    }
}
