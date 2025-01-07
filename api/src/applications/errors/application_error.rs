use thiserror::Error;

use crate::applications::validation::validation_failure::ValidationFailure;
use super::repository_error::RepositoryError;

#[derive(Error, Debug)]
pub enum ApplicationError {
  #[error("Repository error: {0}")]
  RepositoryError(#[from] RepositoryError),
  #[error("Validation error: {0:?}")]
  ValidationError(ValidationFailure),
  #[error("Unexpected error: {0}")]
  UnexpectedError(String),
  #[error("Conflict error: {0}")]
  ConflictError(String),
}