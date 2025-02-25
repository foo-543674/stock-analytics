use axum::async_trait;
use sea_orm::*;
use std::sync::Arc;

pub const IN_MEMORY_SQLITE: &str = "sqlite::memory:";

pub fn create_db_options(url: impl Into<String>, max_connection: u32) -> ConnectOptions {
  let mut opt = ConnectOptions::new(url);
  opt
    .max_connections(max_connection)
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Debug);
  opt
}

pub async fn create_db_connection(opt: &ConnectOptions) -> Result<DatabaseConnection, DbErr> {
  Database::connect(opt.clone()).await
}

#[async_trait]
pub trait ConnectionProvider: Sync + Send {
  async fn provide(&self) -> Result<Arc<DatabaseConnection>, DbErr>;
}

pub struct ConnectionProviderImpl {
  option: ConnectOptions,
}

impl ConnectionProviderImpl {
  pub fn new(option: ConnectOptions) -> Self {
    Self { option }
  }
}

#[async_trait]
impl ConnectionProvider for ConnectionProviderImpl {
  async fn provide(&self) -> Result<Arc<DatabaseConnection>, DbErr> {
    let connection = create_db_connection(&self.option).await?;
    return Ok(Arc::new(connection));
  }
}
