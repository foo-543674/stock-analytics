use crate::applications::errors::application_error::ApplicationError;
use super::validation_failure::ValidationFailure;

macro_rules! validation_error {
  ($failure:expr) => {
    ApplicationError::ValidationError($failure)
  };

  ($field:literal, $($keys:expr),+) => {
    ApplicationError::ValidationError(
      crate::applications::validation::validation_failure::validation_failure!($field, $($keys),+)
    )
  };

  ($field:expr, $($keys:expr),+) => {
    ApplicationError::ValidationError(
      crate::applications::validation::validation_failure::validation_failure!($field, $($keys),+)
    )
  };

  (@list $($fields:expr),+) => {{
    ApplicationError::ValidationError(
      crate::applications::validation::validation_failure::validation_failure!(@list $($fields),+)
    )
  }};
}

pub(crate) use validation_error;

pub trait ResultExt<T, E> {
  fn is_validation_error_and_has_field(&self, field: &str) -> bool;
  fn add_validation_failure(self, other: &ValidationFailure) -> Result<T, E>;
  fn combine_with(self, other: Result<T, E>) -> Result<T, E>;
}

impl<T> ResultExt<T, ApplicationError> for Result<T, ApplicationError> {
  fn is_validation_error_and_has_field(&self, field: &str) -> bool {
    match self {
      Ok(_) => false,
      Err(ApplicationError::ValidationError(failure)) => failure.has_field(field),
      Err(_) => false,
    }
  }

  fn add_validation_failure(self, other: &ValidationFailure) -> Result<T, ApplicationError> {
    match self {
      Ok(_) => Err(validation_error!(other.clone())),
      Err(ApplicationError::ValidationError(failure)) => Err(validation_error!(failure.merge(other))),
      Err(error) => Err(error),
    }
  }

  fn combine_with(self, other: Result<T, ApplicationError>) -> Result<T, ApplicationError> {
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::applications::validation::{
    validation_failure::validation_failure,
    validation_message_keys::{
      numeric_only,
      required
    }
  };

  #[test]
  fn is_validation_error_and_has_field_return_true_when_field_exists() {
    let result: Result<(), ApplicationError> = Err(validation_error!("field", required()));
    assert!(result.is_validation_error_and_has_field("field"));
  }

  #[test]
  fn is_validation_error_and_has_field_return_false_when_field_does_not_exist() {
    let result: Result<(), ApplicationError> = Err(validation_error!("field", required()));
    assert!(!result.is_validation_error_and_has_field("another_field"));
  }

  #[test]
  fn add_validation_failure_overwrite_ok_when_other_is_err() {
    let result: Result<(), ApplicationError> = Ok(());
    let failure = validation_failure!("field", required());
    let result = result.add_validation_failure(&failure);
    if let Err(ApplicationError::ValidationError(failure)) = result {
      assert!(failure.has_field("field"));
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn add_validation_failure_add_err_when_other_is_err() {
    let result: Result<(), ApplicationError> = Err(validation_error!("field1", required()));
    let failure = validation_failure!("field2", numeric_only());
    let result = result.add_validation_failure(&failure);
    if let Err(ApplicationError::ValidationError(failure)) = result {
      assert!(failure.has_field("field1"));
      assert!(failure.has_field("field2"));
      assert_eq!(failure.fields_count(), 2);
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn add_validation_failure_return_as_is_when_not_validation_error() {
    let result: Result<(), ApplicationError> = Err(ApplicationError::UnexpectedError("unexpected".to_string()));
    let failure = validation_failure!("field", numeric_only());
    let result = result.add_validation_failure(&failure);
    if let Err(ApplicationError::UnexpectedError(message)) = result {
      assert_eq!(message, "unexpected");
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn combine_with_return_ok_when_both_ok_and_value_not_change() {
    let result1: Result<String, ApplicationError> = Ok("before".to_string());
    let result2: Result<String, ApplicationError> = Ok("after".to_string());
    let result = result1.combine_with(result2);
    if let Ok(value) = result {
      assert_eq!(value, "before");
    } else {
      panic!("Expected Ok but got {:?}", result);
    }
  }

  #[test]
  fn combine_with_return_merged_validation_error_when_both_validation_error() {
    let result1: Result<String, ApplicationError> = Err(validation_error!("field1", required()));
    let result2: Result<String, ApplicationError> = Err(validation_error!("field2", numeric_only()));
    let result = result1.combine_with(result2);
    if let Err(ApplicationError::ValidationError(failure)) = result {
      assert!(failure.has_field("field1"));
      assert!(failure.has_field("field2"));
      assert_eq!(failure.fields_count(), 2);
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn combine_with_overwrite_ok_to_validation_error() {
    let result1: Result<String, ApplicationError> = Ok("before".to_string());
    let result2: Result<String, ApplicationError> = Err(validation_error!("field", required()));
    let result = result1.combine_with(result2);
    if let Err(ApplicationError::ValidationError(failure)) = result {
      assert!(failure.has_field("field"));
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn combine_with_return_as_is_when_not_validation_error() {
    let result1: Result<String, ApplicationError> = Err(ApplicationError::UnexpectedError("unexpected".to_string()));
    let result2: Result<String, ApplicationError> = Err(validation_error!("field", required()));
    let result = result1.combine_with(result2);
    if let Err(ApplicationError::UnexpectedError(message)) = result {
      assert_eq!(message, "unexpected");
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }

  #[test]
  fn combine_with_overwrite_ok_when_other_is_error_but_not_validation_error() {
    let result1: Result<String, ApplicationError> = Ok("before".to_string());
    let result2: Result<String, ApplicationError> = Err(ApplicationError::UnexpectedError("unexpected".to_string()));
    let result = result1.combine_with(result2);
    if let Err(ApplicationError::UnexpectedError(message)) = result {
      assert_eq!(message, "unexpected");
    } else {
      panic!("Expected error but got {:?}", result);
    }
  }
}