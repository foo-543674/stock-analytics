use async_trait::async_trait;
use regex::Regex;
use crate::applications::{
  common::resource_key::resource_key,
  errors::application_error::ApplicationError,
  validation::{
    simple_validator::SimpleValidator,
    validation_failure::validation_failure,
    validator::Validator,
    validator_chain::ValidatorChain
  }
};

pub struct BrandRegisterInput {
  pub name: String,
  pub code: String,
  pub sector_id: String,
}

pub struct BrandRegisterInputValidator {
  base_validator: ValidatorChain<BrandRegisterInput>,
}

const CODE_LENGTH: usize = 4;

impl BrandRegisterInputValidator {
  pub fn new() -> Self {
    BrandRegisterInputValidator {
      base_validator: ValidatorChain::new(vec![
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.name.is_empty(),
          || validation_failure!("name", "required"))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.code.is_empty(),
          || validation_failure!("code", "required"))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.code.len() == CODE_LENGTH,
          || validation_failure!("code", "length_match", CODE_LENGTH))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| Regex::new(r"^\d+$").unwrap().is_match(&target.code),
          || validation_failure!("code", "numeric_only"))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.sector_id.is_empty(),
          || validation_failure!("sector_id", "required"))
        ),
      ]),
    }
  }
}

#[async_trait]
impl Validator<BrandRegisterInput> for BrandRegisterInputValidator {
  async fn validate(&self, target: &BrandRegisterInput) -> Result<(), ApplicationError> {
    self.base_validator.validate(target).await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;
  use crate::test_support::{
    string::{
      empty,
      random_text,
      alphanumeric_string,
      fixed_length_numeric_string,
      fixed_length_numeric_string_except
    },
    unambiguous_ulid::unambiguous_ulid
  };

  proptest! {
    #[test]
    fn brand_register_input_validator_should_return_error_when_name_is_empty(
      name in empty(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_err());
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_empty(
      name in random_text(),
      code in empty(),
      sector_id in unambiguous_ulid()
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_err());
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_length_does_not_4(
      name in random_text(),
      code in fixed_length_numeric_string_except(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_err());
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_not_numeric(
      name in random_text(),
      code in alphanumeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_err());
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_id_is_empty(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH)
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id: "".to_string(),
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_err());
    }

    #[test]
    fn brand_register_input_validator_should_return_ok_when_all_fields_are_not_satisfy_following_conditions(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let validator = BrandRegisterInputValidator::new();
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert!(result.is_ok());
    }
  }
}