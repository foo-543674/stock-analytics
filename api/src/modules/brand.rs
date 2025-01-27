use std::sync::Arc;

use sea_orm::ConnectOptions;

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
  infrastructures::{brand::{
    brand_list_query::BrandListQuery, brand_repository::BrandRepositoryOnMemory, secrot_repository::SectorRepositoryOnMemory
  }, support::connection::create_db_connection}
};

use super::errors::ModuleError;

pub struct BrandModule {
  db_option: Arc<ConnectOptions>
}

impl BrandModule {
  pub fn new(db_option: Arc<ConnectOptions>) -> Self {
    Self { db_option }
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

  pub async fn resolve_brand_list_query(&self) -> Result<BrandListQuery, ModuleError> {
    let db_connection = Arc::new(create_db_connection(&self.db_option).await?);

    Ok(BrandListQuery::new(db_connection))
  }
}