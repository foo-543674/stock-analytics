use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ServerConfig {
  pub host: String,
  pub port: u16,
}
