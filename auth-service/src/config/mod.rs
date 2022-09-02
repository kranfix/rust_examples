pub mod crypto;

use std::time::Duration;

use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;
use tracing_subscriber::EnvFilter;

use self::crypto::CryptoService;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub host: String,
  pub port: i32,
  pub database_url: String,
  pub secret_key: String,
}

impl Config {
  #[tracing::instrument]
  pub fn from_env() -> Result<Config> {
    dotenv().ok();

    tracing_subscriber::fmt()
      .with_env_filter(EnvFilter::from_default_env())
      .init();

    info!("Loading configuration");

    let mut c = config::Config::new();
    c.merge(config::Environment::default())?;

    c.try_into()
      .context("loading configuration from environment")
  }

  pub async fn db_pool(&self) -> Result<PgPool> {
    info!("Creting database connection pool.");

    PgPoolOptions::new()
      .acquire_timeout(Duration::from_secs(30))
      .connect(&self.database_url)
      .await
      .context("Creating database connection pool")
  }

  pub fn crypto_service(&self) -> CryptoService {
    CryptoService {
      key: std::sync::Arc::new(self.secret_key.clone()),
    }
  }
}
