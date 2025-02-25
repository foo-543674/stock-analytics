use std::sync::Arc;

use crate::applications::brand::interactors::brand_factory::BrandFactory;
use crate::applications::brand::interactors::brand_register_input::BrandRegisterInput;
use crate::applications::brand::interactors::brand_register_input_validator::ValidatedBrandRegisterInput;
use crate::applications::brand::repositories::brand_repository::BrandRepository;
use crate::applications::errors::application_error::ApplicationError;
use crate::applications::validation::validator::Validator;
use crate::domains::brand::brand::Brand;

pub struct RegisterBrandUsecase {
  brand_repository: Arc<dyn BrandRepository>,
  brand_register_input_validator:
    Arc<dyn Validator<BrandRegisterInput, ValidatedBrandRegisterInput>>,
  brand_factory: Arc<dyn BrandFactory>,
}

impl RegisterBrandUsecase {
  pub fn new(
    brand_repository: Arc<dyn BrandRepository>,
    brand_register_input_validator: Arc<
      dyn Validator<BrandRegisterInput, ValidatedBrandRegisterInput>,
    >,
    brand_factory: Arc<dyn BrandFactory>,
  ) -> Self {
    Self {
      brand_repository,
      brand_register_input_validator,
      brand_factory,
    }
  }

  pub async fn execute(&self, input: BrandRegisterInput) -> Result<Brand, ApplicationError> {
    let validated_input = self.brand_register_input_validator.validate(&input).await?;
    let brand = self.brand_factory.create(&validated_input)?;
    self.brand_repository.add(&brand).await?;
    Ok(brand)
  }
}

#[cfg(test)]
mod test {
  use crate::{
    applications::{
      brand::{
        interactors::{
          brand_factory::MockBrandFactory, brand_register_input::BrandRegisterInput,
          brand_register_input_validator::ValidatedBrandRegisterInput,
        },
        repositories::brand_repository::MockBrandRepository,
        usecases::register_brand_usecase::RegisterBrandUsecase,
      },
      errors::application_error::ApplicationError,
      validation::{
        validation_error::validation_error, validation_message_keys::required,
        validator::MockValidator,
      },
    },
    domains::brand::brand::Brand,
    test_support::mock::*,
  };
  use std::sync::Arc;

  #[tokio::test]
  async fn register_brand_usecase_should_save_brand_when_validator_resutn_success() {
    let input = BrandRegisterInput::new("name", "code", "sector_id");
    let brand = Brand::default();
    let brand_repository = create_mock::<MockBrandRepository>(|mock| {
      once_returning!(mock, expect_add, box_ok!(()));
    });
    let validator =
      create_mock::<MockValidator<BrandRegisterInput, ValidatedBrandRegisterInput>>(|mock| {
        once_returning!(mock, expect_validate, |input| {
        let result = ValidatedBrandRegisterInput::new(input, Default::default());
        box_ok!(@move result)
      } => closure);
      });
    let brand_factory = create_mock::<MockBrandFactory>(|mock| {
      once_returning!(mock, expect_create, Ok(Default::default()));
    });

    let usecase = RegisterBrandUsecase::new(
      Arc::new(brand_repository),
      Arc::new(validator),
      Arc::new(brand_factory),
    );

    let result = usecase.execute(input).await;

    assert!(result.is_ok());
    assert_eq!(result.expect("must be success"), brand);
  }

  #[tokio::test]
  async fn register_brand_usecase_should_not_save_when_validator_return_error() {
    let brand_repository = create_mock::<MockBrandRepository>(|mock| {
      do_not_call!(mock, expect_add);
    });
    let validator =
      create_mock::<MockValidator<BrandRegisterInput, ValidatedBrandRegisterInput>>(|mock| {
        once_returning!(
          mock,
          expect_validate,
          box_err!(validation_error!("foo", required()))
        );
      });
    let brand_factory = create_mock::<MockBrandFactory>(|mock| {
      do_not_call!(mock, expect_create);
    });

    let usecase = RegisterBrandUsecase::new(
      Arc::new(brand_repository),
      Arc::new(validator),
      Arc::new(brand_factory),
    );

    let input = BrandRegisterInput::new("name", "code", "sector_id");
    let result = usecase.execute(input).await;

    assert!(result.is_err());
  }
}
