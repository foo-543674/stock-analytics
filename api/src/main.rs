use std::{env, str::FromStr};
use stock_analytics::config::app_config::AppConfig;
use stock_analytics::{modules::root::RootModule, routes::router::router};

#[tokio::main]
async fn main() {
  let mode = env::var("APP__MODE").unwrap_or_else(|_| "dev".into());
  let config = AppConfig::load(&mode).expect("Failed to load config");

  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::from_str(&config.logging.level).unwrap_or(tracing::Level::INFO))
    .init();

  let module = RootModule::new(&config);

  let listener =
    tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port))
      .await
      .unwrap_or_else(|_| panic!("Failed to bind to port {}", config.server.port));

  tracing::info!(
    "Server started on {}:{}",
    config.server.host,
    config.server.port
  );
  axum::serve(listener, router(module))
    .await
    .expect("Failed to start server");
}
