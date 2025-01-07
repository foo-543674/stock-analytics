use std::sync::Arc;
use async_trait::async_trait;
use futures::{future::FutureExt, TryFutureExt};

use crate::{
  applications::{
    brand::repositories::{
      brand_repository::BrandRepository,
      sector_repository::SectorRepository
    }, 
    errors::application_error::ApplicationError, 
    validation::{
      simple_validator::SimpleValidator, validation_error::{
        validation_error, ResultExt
      }, validation_failure::validation_failure, validation_message_keys::{
        duplicate, length_equals, max_length, numeric_only, required, resource_not_found, ulid
      },
      validator::Validator,
      validator_chain::ValidatorChain
    }
  }, 
  domains::brand::{
    brand::BrandCode,
    sector::{Sector, SectorId}
  },
  util::{
    str::StringExt, 
    ulid::StringExtForUlid
  }
};

use super::brand_register_input::BrandRegisterInput;

pub struct BrandRegisterInputValidator {
  base_validator: ValidatorChain<BrandRegisterInput, ()>,
  brand_repository: Arc<dyn BrandRepository>,
  sector_repository: Arc<dyn SectorRepository>,
}

const NAME_MAX_LENGTH: usize = 100;

impl BrandRegisterInputValidator {
  pub fn new(
    brand_repository: Arc<dyn BrandRepository>,
    sector_repository: Arc<dyn SectorRepository>,
  ) -> Self {
    BrandRegisterInputValidator {
      base_validator: ValidatorChain::new(vec![
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.name.is_empty(),
          || validation_failure!("name", required()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.name.len() <= NAME_MAX_LENGTH,
          || validation_failure!("name", max_length(NAME_MAX_LENGTH)))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| !target.code.is_empty(),
          || validation_failure!("code", required()))
        ),
        Box::new(SimpleValidator::new(
          |target: &BrandRegisterInput| target.code.len() == BrandCode::BRAND_CODE_LENGTH,
          || validation_failure!("code", length_equals(BrandCode::BRAND_CODE_LENGTH)))
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

pub struct BrandRegisterInputValidationSuccess {
  pub found_sector: Sector,
}

#[async_trait]
impl Validator<BrandRegisterInput, BrandRegisterInputValidationSuccess> for BrandRegisterInputValidator {
  async fn validate(&self, target: &BrandRegisterInput) -> Result<BrandRegisterInputValidationSuccess, ApplicationError> {
    return self.base_validator.validate(target)
      .then(|result| async {
        // NOTE: If validation error for code is already exists, BrandCode cannot construct.
        if result.is_validation_error_and_has_field("code") {
          result
        } else {
          let brand = self.brand_repository.find_by_code(&BrandCode::from_string(&target.code)).await?;
          match brand {
            Some(_) => result.add_validation_failure(&validation_failure!("code", duplicate())),
            None => result,
          }
        }
      })
      .then(|result| async {
        // NOTE: If validation error for sector_id is already exists, SectorId cannot construct.
        if result.is_validation_error_and_has_field("sector_id") {
          Err(result.unwrap_err())
        } else {
          let sector_option = self.sector_repository.find_by_id(&SectorId::from_string(&target.sector_id)).await?;
          match (result, sector_option) {
            (Ok(_), Some(sector)) => Ok(sector),
            (Ok(_), None) => Err(validation_error!("sector_id", resource_not_found())),
            (Err(ApplicationError::ValidationError(failure)), None) => Err(validation_error!(failure.merge(&validation_failure!("sector_id", resource_not_found())))),
            (Err(err), _) => Err(err),
          }
        }
      })
      .and_then(|sector| async {
        Ok(BrandRegisterInputValidationSuccess {
          found_sector: sector,
        })
      }).await;
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
        random_text_length_at_least,
        random_text_length_at_most,
        random_text
      },
      ulid::random_ulid_string
    }
  };

  proptest! {
    #[test]
    fn brand_register_input_validator_should_return_error_when_name_is_empty(
      name in empty(),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_name_length_is_over_100(
      name in random_text_length_at_least(NAME_MAX_LENGTH + 1),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_empty(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in empty(),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_length_does_not_4(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string_except(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Default::default()) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_code_is_not_numeric(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in alphanumeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(0);
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_id_is_empty(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in empty()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(0);

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_id_is_not_ulid(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_text()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(0);

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_same_code_exists(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_error_when_sector_not_found(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(None) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name,
        code,
        sector_id,
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_err!(result);
    }

    #[test]
    fn brand_register_input_validator_should_return_success_when_all_fields_are_not_satisfy_above_conditions(
      name in random_text_length_at_most(NAME_MAX_LENGTH),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid_string()
    ) {
      let mut brand_repository = MockBrandRepository::new();
      brand_repository.expect_find_by_code().times(1).returning(|_| Box::pin(async { Ok(None) }));
      let mut sector_repository = MockSectorRepository::new();
      sector_repository.expect_find_by_id().times(1).returning(|_| Box::pin(async { Ok(Some(Default::default())) }));

      let sut = BrandRegisterInputValidator::new(Arc::new(brand_repository), Arc::new(sector_repository));
      let input = BrandRegisterInput {
        name: name.clone(),
        code: code.clone(),
        sector_id: sector_id.clone(),
      };
      let result = futures::executor::block_on(sut.validate(&input));
      assert_result_is_ok!(result);
      let success = result.unwrap();
      assert_eq!(success.found_sector, Default::default());
    }
  }
}
