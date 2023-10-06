use crate::configuration::env::Environment;
use anyhow::Result;
use secrecy::Secret;
use serde::Deserialize;
use std::{
    fmt::Debug,
    net::{AddrParseError, SocketAddr},
    str::FromStr,
};
use tap::{Tap, TapFallible};

const BASE_CONFIG_FILENAME: &str = "base.yaml";

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub application_name: String,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

impl TryFrom<&ServerSettings> for SocketAddr {
    type Error = AddrParseError;

    fn try_from(value: &ServerSettings) -> Result<Self, Self::Error> {
        Self::from_str(&format!("{}:{}", value.host, value.port))
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub pool: PollSettings,
    pub require_ssl: bool,
    pub migration: MigrationSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct MigrationSettings {
    pub enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PollSettings {
    pub max_connection: u32,
    pub max_lifetime: u64,
    pub idle_timeout: u64,
    pub acquire_timeout: u64,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let environment = Environment::which();
        tracing::info!(
            "Starting to loading configuration from {} environment",
            environment
        );
        let config_dir = Environment::config_path();
        let environment_filename = format!("{}.yaml", environment).to_lowercase();
        let settings = config::Config::builder()
            .add_source(config::File::from(config_dir.join(BASE_CONFIG_FILENAME)))
            .add_source(config::File::from(config_dir.join(environment_filename)))
            .add_source(
                config::Environment::with_prefix("w")
                    .try_parsing(true)
                    .separator("__")
                    .list_separator(","),
            )
            .build()
            .tap_err(|e| tracing::error!("{:?}", e))?;
        settings
            .try_deserialize::<Self>()
            .tap_err(|e| tracing::error!("Failed to deserialize settings. Error: {e}"))
            .map_err(anyhow::Error::from)
            .tap(|s| tracing::debug!("Settings loaded: {s:#?}"))
    }
}
