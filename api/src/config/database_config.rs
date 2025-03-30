use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DatabaseConfig {
  pub url: String,
  pub pool_size: u32,
}