use config::{Config, ConfigError, Environment, File, FileFormat};
use serde::Deserialize;

use super::{database_config::DatabaseConfig, log_config::LogConfig, server_config::ServerConfig};

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AppConfig {
  pub server: ServerConfig,
  pub database: DatabaseConfig,
  pub logging: LogConfig,
}

impl AppConfig {
  pub fn load(mode: &str) -> Result<Self, ConfigError> {
    let s = Config::builder()
      .add_source(
        File::with_name("conf/app")
          .format(FileFormat::Toml)
          .required(true),
      )
      .add_source(
        File::with_name(&format!("conf/{mode}"))
          .format(FileFormat::Toml)
          .required(false),
      )
      .add_source(Environment::with_prefix("APP").separator("__"))
      .build()?;

    s.try_deserialize()
  }
}
