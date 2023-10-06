use std::time::Duration;

use commons::configuration::settings::Settings;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    ConnectOptions, Error, PgPool,
};

pub mod active;
pub mod model;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

pub trait Entity {
    type Id: PartialEq;

    fn id(&self) -> Self::Id;

    fn same_as(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

pub async fn pool_from_settings(settings: &Settings) -> Result<PgPool, Error> {
    let option = &ConnectionOptions(&settings);
    let pool: PgPoolOptions = option.into();
    pool.connect_with(option.into()).await
}

pub struct ConnectionOptions<'s>(&'s Settings);

impl Into<PgConnectOptions> for &ConnectionOptions<'_> {
    fn into(self) -> PgConnectOptions {
        let database_settings = &self.0.database;
        let ssl_mode = if database_settings.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .application_name(&self.0.application_name)
            .host(&database_settings.host)
            .username(&database_settings.username)
            .password(secrecy::ExposeSecret::expose_secret(
                &database_settings.password,
            ))
            .port(database_settings.port)
            .ssl_mode(ssl_mode)
            .database(&database_settings.database_name)
            .disable_statement_logging()
    }
}

impl Into<PgPoolOptions> for &ConnectionOptions<'_> {
    fn into(self) -> PgPoolOptions {
        let settings = &self.0.database.pool;
        PgPoolOptions::new()
            .max_connections(settings.max_connection)
            .max_lifetime(Some(Duration::from_secs(settings.max_lifetime)))
            .idle_timeout(Some(Duration::from_secs(settings.idle_timeout)))
            .acquire_timeout(Duration::from_secs(settings.acquire_timeout))
            .acquire_timeout(Duration::from_secs(1))
    }
}
