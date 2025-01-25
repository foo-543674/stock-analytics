use sea_orm::*;

pub const IN_MEMORY_SQLITE: &str = "sqlite::memory:";

pub fn create_db_options(url: impl Into<String>, max_connection: u32) -> ConnectOptions {
  let mut opt = ConnectOptions::new(url);
  opt.max_connections(max_connection)
    .sqlx_logging(true)
    .sqlx_logging_level(log::LevelFilter::Debug);
  return opt;
}

pub async fn create_db_connection(opt: ConnectOptions) -> Result<DatabaseConnection, DbErr> {
  return Database::connect(opt).await;
}
