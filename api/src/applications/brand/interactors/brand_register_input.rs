use std::sync::Arc;

use async_trait::async_trait;
use crate::{
  applications::{
    brand::repositories::{
      brand_repository::BrandRepository,
      sector_repository::SectorRepository
    },
    errors::application_error::ApplicationError, 
    validation::{
      simple_validator::SimpleValidator,
      validation_error::{
        validation_error, ResultExt
      },
      validation_failure::validation_failure,
      validation_message_keys::{
        duplicate, length_equals, numeric_only, required, resource_not_found, ulid
      },
      validator::Validator,
      validator_chain::ValidatorChain
    }
  }, 
  domains::brand::{brand::BrandCode, sector::SectorId},
  util::{
    str::StringExt, 
    ulid::StringExtForUlid
  }
};

pub struct BrandRegisterInput {
  pub name: String,
  pub code: String,
  pub sector_id: String,
}

pub struct BrandRegisterInputValidator {
  base_validator: ValidatorChain<BrandRegisterInput>,
  brand_repository: Arc<dyn BrandRepository>,
  sector_repository: Arc<dyn SectorRepository>,
}

const CODE_LENGTH: usize = 4;

impl BrandRegisterInputValidator {
  pub fn new(
    brand_repository: Arc<dyn BrandRepository>,
    sector_repository: Arc<dyn SectorRepository>
  ) -> Self {
    BrandRegisterInputValidator {
      base_validator: ValidatorChain::new(vec![
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.name.is_empty(),
          || validation_failure!("name", required()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.code.is_empty(),
          || validation_failure!("code", required()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.code.len() == CODE_LENGTH,
          || validation_failure!("code", length_equals(CODE_LENGTH)))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.code.is_numeric(),
          || validation_failure!("code", numeric_only()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.sector_id.is_empty(),
          || validation_failure!("sector_id", required()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.sector_id.is_ulid(),
          || validation_failure!("sector_id", ulid()))
        ),
      ]),
      brand_repository,
      sector_repository,
    }
  }
}

#[async_trait]
impl Validator<BrandRegisterInput> for BrandRegisterInputValidator {
  async fn validate(&self, target: &BrandRegisterInput) -> Result<(), ApplicationError> {
    let base_result = self.base_validator.validate(target).await;

    let code_dupulicate_validation_result = if base_result.is_validation_error_and_has_field("code")  {
      Ok(())
    } else {
      let brand = self.brand_repository.find_by_code(&BrandCode::from_string(&target.code)).await?;
      match brand {
        Some(_) => Err(validation_error!("code", duplicate())),
        None => Ok(()),
      }
    };

    let sector_exists_validation_result = if base_result.is_validation_error_and_has_field("sector_id") {
      Ok(())
    } else {
      let sector = self.sector_repository.find_by_id(&SectorId::from_string(&target.sector_id)).await?;
      match sector {
        Some(_) => Ok(()),
        None => Err(validation_error!("sector_id", resource_not_found())),
      }
    };

    base_result
      .merge_or_overwrite_when_either_error(code_dupulicate_validation_result)
      .merge_or_overwrite_when_either_error(sector_exists_validation_result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;
  use crate::{
    applications::brand::repositories::{
      brand_repository::MockBrandRepository, 
      sector_repository::MockSectorRepository
    },
    test_support::{
      assert_result::{
        assert_result_is_err, 
        assert_result_is_ok
      },
      string::{
        alphanumeric_string,
        empty,
        fixed_length_numeric_string,
        fixed_length_numeric_string_except,
        random_text
      },
      unambiguous_ulid::unambiguous_ulid
    }
  };

  proptest! {
    #[test]
    fn brand_register_input_validator_should_return_error_when_name_is_empty(
      name in empty(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_empty(
      name in random_text(),
      code in empty(),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_length_does_not_4(
      name in random_text(),
      code in fixed_length_numeric_string_except(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_not_numeric(
      name in random_text(),
      code in alphanumeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_id_is_empty(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH)
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(0);

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id: "".to_string(),
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_id_is_not_ulid(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in random_text()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(0);

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_same_code_exists(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_not_found(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(None) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_ok_when_all_fields_are_not_satisfy_above_conditions(
      name in random_text(),
      code in fixed_length_numeric_string(CODE_LENGTH),
      sector_id in unambiguous_ulid()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let validator = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(validator.validate(&input));
      assert_result_is_ok!(result);
    }
  }
}