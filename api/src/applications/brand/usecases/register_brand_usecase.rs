use std::sync::Arc;

use crate::applications::brand::interactors::brand_factory::BrandFactory;
use crate::applications::brand::interactors::brand_register_input::BrandRegisterInput;
use crate::applications::brand::repositories::brand_repository::BrandRepository;
use crate::applications::brand::interactors::brand_register_input_validator::BrandRegisterInputValidationSuccess;
use crate::applications::errors::application_error::ApplicationError;
use crate::applications::validation::validator::Validator;
use crate::domains::brand::brand::Brand;

pub struct RegisterBrandUsecase {
  brand_repository: Arc<dyn BrandRepository>,
  brand_register_input_validator: Arc<dyn Validator<BrandRegisterInput, BrandRegisterInputValidationSuccess>>,
  brand_factory: Arc<dyn BrandFactory>,
}

impl RegisterBrandUsecase {
  pub fn new(
    brand_repository: Arc<dyn BrandRepository>,
    brand_register_input_validator: Arc<dyn Validator<BrandRegisterInput, BrandRegisterInputValidationSuccess>>, 
    brand_factory: Arc<dyn BrandFactory>,
  ) -> Self {
    Self {
      brand_repository,
      brand_register_input_validator,
      brand_factory,
    }
  }

  pub async fn execute(&self, input: BrandRegisterInput) -> Result<Brand, ApplicationError> {
    let validation_success = self.brand_register_input_validator.validate(&input).await?;
    let brand = self.brand_factory.create(&input, &validation_success)?;
    self.brand_repository.add(&brand).await?;
    Ok(brand)
  }
}

#[cfg(test)]
mod test {
  use std::sync::Arc;
  use crate::{
    applications::{
      brand::{
        interactors::{
          brand_factory::MockBrandFactory, 
          brand_register_input::BrandRegisterInput, 
          brand_register_input_validator::BrandRegisterInputValidationSuccess
        }, 
        repositories::brand_repository::MockBrandRepository, 
        usecases::register_brand_usecase::RegisterBrandUsecase
      }, 
      errors::application_error::ApplicationError, validation::{
        validation_error::validation_error, 
        validation_message_keys::required, 
        validator::MockValidator
      }
    }, 
    domains::brand::brand::{
      Brand, 
      BrandCode, 
      BrandId
    }, 
    util::{
      ulid::default_ulid, 
      unempty_string::UnemptyString
    }
  };

  #[tokio::test]
  async fn register_brand_usecase_should_save_brand_when_validator_resutn_success() {
    let brand = Brand {
      id: BrandId::new(default_ulid()),
      name: UnemptyString::from_string("name"),
      code: BrandCode::from_string("0000"),
      sector: Default::default(),
    };

    let mut brand_repository = MockBrandRepository::new();
    brand_repository.expect_add().times(1).returning(|_| Box::pin(async { Ok(()) }));

    let mut brand_register_input_validator = MockValidator::new();
    brand_register_input_validator.expect_validate().times(1).returning(move |_| {
      Box::pin(async { Ok(BrandRegisterInputValidationSuccess{
        found_sector: Default::default(),
      }) })
    });

    let mut brand_factory = MockBrandFactory::new();
    let brand_clone = brand.clone();
    brand_factory.expect_create().times(1).returning(move |_, _| Ok(brand_clone.clone()));

    let usecase = RegisterBrandUsecase::new( 
      Arc::new(brand_repository),
      Arc::new(brand_register_input_validator),
      Arc::new(brand_factory),
    );

    let input = BrandRegisterInput {
      name: "name".to_string(),
      code: "code".to_string(),
      sector_id: "sector_id".to_string(),
    };
    let result = usecase.execute(input).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), brand);
  }

  #[tokio::test]
  async fn register_brand_usecase_should_not_save_when_validator_return_error() {
    let mut brand_repository = MockBrandRepository::new();
    brand_repository.expect_add().times(0);
    let mut brand_register_input_validator = MockValidator::new();
    brand_register_input_validator.expect_validate().times(1).returning(move |_| {
      Box::pin(async { Err(validation_error!("foo", required())) })
    });
    let mut brand_factory = MockBrandFactory::new();
    brand_factory.expect_create().times(0);

    let usecase = RegisterBrandUsecase::new( 
      Arc::new(brand_repository),
      Arc::new(brand_register_input_validator),
      Arc::new(brand_factory),
    );

    let input = BrandRegisterInput {
      name: "name".to_string(),
      code: "code".to_string(),
      sector_id: "sector_id".to_string(),
    };
    let result = usecase.execute(input).await;

    assert!(result.is_err());
  }
}