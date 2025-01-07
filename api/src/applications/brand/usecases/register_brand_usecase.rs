use std::sync::Arc;
use crate::applications::brand::interactors::brand_register_input::BrandRegisterInput;
use crate::applications::brand::repositories::brand_repository::BrandRepository;
use crate::applications::brand::interactors::brand_factory::BrandFactory;
use crate::applications::errors::application_error::ApplicationError;

pub struct RegisterBrandUsecase {
  brand_repository: Arc<dyn BrandRepository>,
  brand_factory: BrandFactory,
}

impl RegisterBrandUsecase {
  pub fn new(brand_repository: Arc<dyn BrandRepository>, brand_factory: BrandFactory) -> Self {
    Self {
      brand_repository,
      brand_factory,
    }
  }

  pub async fn execute(&self, input: BrandRegisterInput) -> Result<(), ApplicationError> {
    let brand = self.brand_factory.create(&input).await?;
    self.brand_repository.add(&brand).await?;
    Ok(())
  }
}