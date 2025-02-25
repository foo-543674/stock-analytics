use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
  #[error("Invalid data: {0}")]
  InvalidData(String),
}
