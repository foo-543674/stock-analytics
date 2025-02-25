use async_trait::async_trait;

use super::{
  validation_error::validation_error, validation_failure::ValidationFailure, validator::Validator,
};
use crate::applications::errors::application_error::ApplicationError;

pub struct SimpleValidator<T> {
  pub f: fn(&T) -> bool,
  pub when_error: fn() -> ValidationFailure,
}

impl<T> SimpleValidator<T> {
  pub fn new(f: fn(&T) -> bool, when_error: fn() -> ValidationFailure) -> Self {
    SimpleValidator { f, when_error }
  }
}

#[async_trait]
impl<T: Send + Sync> Validator<T, ()> for SimpleValidator<T> {
  async fn validate(&self, target: &T) -> Result<(), ApplicationError> {
    if (self.f)(target) {
      Ok(())
    } else {
      Err(validation_error!((self.when_error)()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::applications::validation::{
    validation_failure::validation_failure, validation_message_keys::required,
  };

  #[test]
  fn simple_validator_should_return_ok_when_true() {
    let validator = SimpleValidator::new(|_| true, || validation_failure!("field", required()));
    let result = futures::executor::block_on(validator.validate(&"target"));
    assert!(result.is_ok());
  }

  #[test]
  fn simple_validator_should_return_error_when_false() {
    let validator = SimpleValidator::new(|_| false, || validation_failure!("field", required()));
    let result = futures::executor::block_on(validator.validate(&"target"));
    assert!(result.is_err());
  }
}
