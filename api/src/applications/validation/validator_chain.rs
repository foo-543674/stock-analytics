use futures::StreamExt;

use crate::applications::errors::application_error::ApplicationError;
use super::{validation_failure::ValidationFailure, validator::Validator};

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
    let results = futures::stream::iter(&self.validators)
      .then(|validator| validator.validate(target))
      .fold(Ok(()), |acc, result| async {
        match (acc, result) {
          (Ok(_), Ok(_)) => Ok(()),
          (Err(err), Ok(_)) => Err(err),
          (Ok(_), Err(ApplicationError::ValidationError(failures))) => {
            Err(ApplicationError::ValidationError(failures))
          }
          (Err(ApplicationError::ValidationError(prev_failures)), Err(ApplicationError::ValidationError(failures))) => {
            Err(ApplicationError::ValidationError(prev_failures.into_iter().chain(failures.into_iter()).collect()))
          }
          (_, Err(err)) => Err(err),
        }
      }).await;

    match results {
      Ok(_) => Ok(()),
      Err(ApplicationError::ValidationError(failures)) => {
        let merged_failures = failures.into_iter().fold(vec![] as Vec<ValidationFailure>, |acc, failure| {
          if let Some(same_key_failure) = acc.iter().find(|f| f.field == failure.field) {
            let merged = same_key_failure.merge(&failure);
            acc.iter().filter(|f| f.field != failure.field).cloned().chain(vec![merged]).collect()
          } else {
            acc.into_iter().chain(vec![failure]).collect()
          }
        });
        Err(ApplicationError::ValidationError(merged_failures))
      },
      Err(err) => Err(err)
    }
  }
}

#[cfg(test)]
mod tests {
    use std::iter;
    use crate::applications::{common::resource_key::resource_key, validation::{validation_failure::validation_failure, validator::MockValidator}};

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
    assert!(result.is_ok());
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
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Err(ApplicationError::ValidationError(vec![])) }));
        mock
      }))
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert!(result.is_err());
  }

  #[test]
  fn validator_chain_should_merge_failures() {
    use super::*;

    let validators = ["key1", "key2", "key3"].into_iter()
      .map(|k| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { 
          Err(ApplicationError::ValidationError(vec![validation_failure!(k, "foo")]))
        }));
        mock
      })
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert!(result.is_err());
    if let Err(ApplicationError::ValidationError(failures)) = result {
      assert_eq!(failures.len(), 3);
    } else {
      panic!("Unexpected error type");
    }
  }

  #[test]
  fn validator_chain_should_merge_validation_failures_with_same_key() {
    use super::*;

    let validators = ["key1", "key1", "key1"].into_iter()
      .map(|k| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { 
          Err(ApplicationError::ValidationError(vec![validation_failure!(k, "foo")]))
        }));
        mock
      })
      .map(|v| Box::new(v) as Box<dyn Validator<_>>)
      .collect::<Vec<_>>();
    let chain = ValidatorChain::new(validators);
    let result = futures::executor::block_on(chain.validate(&"foo".to_string()));
    assert!(result.is_err());
    if let Err(ApplicationError::ValidationError(failures)) = result {
      assert_eq!(failures.len(), 1);
      assert_eq!(failures[0].keys.len(), 3);
    } else {
      panic!("Unexpected error type");
    }
  }

  #[test]
  fn validator_chain_should_throw_away_all_validation_failures_if_any_other_error_occurs() {
    use super::*;

    let validators = iter::repeat_with(|| {
        let mut mock = MockValidator::new();
        mock.expect_validate().times(1).returning(|_| Box::pin(async { Err(ApplicationError::ValidationError(vec![])) }));
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
    assert!(result.is_err());
    if let Err(ApplicationError::UnexpectedError(_)) = result {
      assert!(true);
    } else {
      panic!("Unexpected error type");
    }
  }
}