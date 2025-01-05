use crate::applications::errors::application_error::ApplicationError;
use super::validation_failure::ValidationFailure;

macro_rules! validation_error {
  ($failure:expr) => {
    ApplicationError::ValidationError($failure)
  };

  ($field:literal, $($keys:expr),+) => {
    ApplicationError::ValidationError(
      validation_failure!($field, $($keys),+)
    )
  };

  ($field:expr, $($keys:expr),+) => {
    ApplicationError::ValidationError(
      validation_failure!($field, $($keys),+)
    )
  };

  (@list $($fields:expr),+) => {{
    ApplicationError::ValidationError(
      validation_failure!(@list $($fields),+)
    )
  }};
}

pub(crate) use validation_error;

pub trait ResultExt<T, E> {
  fn is_validation_error_and_has_field(&self, field: &str) -> bool;
  fn add_or_overwrite_validation_failure(self, other: &ValidationFailure) -> Result<T, E>;
  fn merge_or_overwrite_when_either_error(self, other: Result<T, E>) -> Result<T, E>;
}

impl<T> ResultExt<T, ApplicationError> for Result<T, ApplicationError> {
  fn is_validation_error_and_has_field(&self, field: &str) -> bool {
    match self {
      Ok(_) => false,
      Err(ApplicationError::ValidationError(failure)) => failure.has_field(field),
      Err(_) => false,
    }
  }

  fn add_or_overwrite_validation_failure(self, other: &ValidationFailure) -> Result<T, ApplicationError> {
    match self {
      Ok(_) => Err(validation_error!(other.clone())),
      Err(ApplicationError::ValidationError(failure)) => Err(validation_error!(failure.merge(other))),
      Err(error) => Err(error),
    }
  }

  fn merge_or_overwrite_when_either_error(self, other: Result<T, ApplicationError>) -> Result<T, ApplicationError> {
    match (self, other) {
      (Err(ApplicationError::ValidationError(self_error)), Err(ApplicationError::ValidationError(failures))) => {
        Err(validation_error!(self_error.merge(&failures)))
      }
      (Ok(value), Ok(_)) => Ok(value),
      (Ok(_), Err(ApplicationError::ValidationError(failures))) => Err(validation_error!(failures)),
      (Err(err), _) => Err(err),
      (_, Err(err)) => Err(err),
    }
  }
}
