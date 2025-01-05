use futures::StreamExt;

use crate::applications::errors::application_error::ApplicationError;
use super::{validation_error::validation_error, validator::Validator};

pub struct ValidatorChain<T> {
  validators: Vec<Box<dyn Validator<T>>>
}

impl<T: Send + Sync> ValidatorChain<T> {
  pub fn new(validators: Vec<Box<dyn Validator<T>>>) -> Self {
    ValidatorChain {
      validators
    }
  }

  pub async fn validate(&self, target: &T) -> Result<(), ApplicationError> {
    return futures::stream::iter(&self.validators)
      .then(|validator| validator.validate(target))
      .fold(Ok(()), |acc, result| async {
        match (acc, result) {
          (Ok(_), Ok(_)) => Ok(()),
          (Err(err), Ok(_)) => Err(err),
          (Ok(_), Err(ApplicationError::ValidationError(failures))) => {
            Err(validation_error!(failures))
          }
          (Err(ApplicationError::ValidationError(prev_failures)), Err(ApplicationError::ValidationError(failures))) => {
            Err(validation_error!(prev_failures.merge(&failures)))
          }
          (_, Err(err)) => Err(err),
        }
      }).await;
  }
}

#[cfg(test)]
mod tests {
    use std::iter;
    use crate::{
      applications::validation::{
        validation_message_keys::required,
        validator::MockValidator
      }, 
      test_support::assert_result::{
        assert_result_is_err, 
        assert_result_is_ok
      }
    };

  #[test]
  fn validator_chain_should_use_all_validators() {
    use super::*;

    let validators = iter::repeat_with(|| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Ok(()) }));
        mock
      })
      .take(3)
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert_result_is_ok!(result);
  }

  #[test]
  fn validator_chain_should_return_error_if_any_validator_fails() {
    use super::*;

    let validators = iter::repeat_with(|| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Ok(()) }));
        mock
      })
      .take(3)
      .chain(iter::once({
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Err(validation_error!("foo", required())) }));
        mock
      }))
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert_result_is_err!(result);
  }

  #[test]
  fn validator_chain_should_merge_failures() {
    use super::*;

    let validators = ["key1", "key2", "key3"].into_iter()
      .map(|k| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { 
          Err(validation_error!(k, required()))
        }));
        mock
      })
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert_result_is_err!(result);
    if let Err(ApplicationError::ValidationError(failures)) = result {
      assert_eq!(failures.fields_count(), 3);
    } else {
      panic!("Unexpected error type");
    }
  }

  #[test]
  fn validator_chain_should_throw_away_all_validation_failures_if_any_other_error_occurs() {
    use super::*;

    let validators = iter::repeat_with(|| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Err(validation_error!("foo", required())) }));
        mock
      })
      .take(3)
      .chain(iter::once({
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Err(ApplicationError::UnexpectedError("foo".to_string())) }));
        mock
      }))
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert_result_is_err!(result);
    if let Err(ApplicationError::UnexpectedError(_)) = result {
      assert!(true);
    } else {
      panic!("Unexpected error type");
    }
  }
}