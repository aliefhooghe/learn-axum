use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingSettings {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub listen: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuthSettings {
    pub issuer_url: String,
    pub client_id: String,
    pub realm: String,
    pub jwks_cache_refresh_interval_sec: Option<u64>,
    pub redirect_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub logging: Option<LoggingSettings>,
    pub oauth: OAuthSettings,
    pub server: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("settings.json").required(false))
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        s.try_deserialize()
    }
}
