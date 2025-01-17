use std::sync::Arc;

use crate::{
  applications::{
    brand::{
      interactors::{
        brand_factory::{
          BrandFactory, 
          BrandFactoryImpl
        }, 
        brand_register_input::BrandRegisterInput, 
        brand_register_input_validator::{
          BrandRegisterInputValidator, 
          ValidatedBrandRegisterInput
        }
      }, 
      repositories::{
        brand_repository::BrandRepository, 
        sector_repository::SectorRepository
      }, 
      usecases::register_brand_usecase::RegisterBrandUsecase
    }, 
    common::ulid_generator::{
      UlidGenerator, 
      UlidGeneratorImpl
    }, 
    validation::validator::Validator
  }, 
  infrastructures::brand::{
    brand_repository::BrandRepositoryOnMemory, 
    secrot_repository::SectorRepositoryOnMemory
  }
};

//TODO: handle without clone
#[derive(Clone)]
pub struct BrandModule {}

impl BrandModule {
  pub fn new() -> Self {
    Self {}
  }

  pub fn resolve_register_brand_usecase(&self) -> RegisterBrandUsecase {
    let brand_repository: Arc<dyn BrandRepository> = Arc::new(BrandRepositoryOnMemory::new());
    let sector_repository: Arc<dyn SectorRepository> = Arc::new(SectorRepositoryOnMemory::new());
    let validator: Arc<dyn Validator<BrandRegisterInput, ValidatedBrandRegisterInput>> = Arc::new(BrandRegisterInputValidator::new(Arc::clone(&brand_repository), sector_repository));
    let ulid_generator: Arc<dyn UlidGenerator> = Arc::new(UlidGeneratorImpl);
    let factory: Arc<dyn BrandFactory> = Arc::new(BrandFactoryImpl::new(ulid_generator));

    RegisterBrandUsecase::new(
      brand_repository,
      validator,
      factory
    )
  }
}