use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LogConfig {
  pub level: String,
}
