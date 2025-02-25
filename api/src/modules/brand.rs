use std::sync::Arc;

use crate::{
  applications::{
    brand::{
      interactors::{
        brand_factory::{BrandFactory, BrandFactoryImpl},
        brand_register_input::BrandRegisterInput,
        brand_register_input_validator::{
          BrandRegisterInputValidator, ValidatedBrandRegisterInput,
        },
      },
      repositories::{brand_repository::BrandRepository, sector_repository::SectorRepository},
      usecases::register_brand_usecase::RegisterBrandUsecase,
    },
    common::ulid_generator::{UlidGenerator, UlidGeneratorImpl},
    validation::validator::Validator,
  },
  infrastructures::{
    brand::{
      command::{
        brand_repository::BrandRepositoryOnRdbms, secrot_repository::SectorRepositoryOnRdbms,
      },
      query::brand_list_query::BrandListQuery,
    },
    support::connection::ConnectionProvider,
  },
};

pub struct BrandModule {
  connection_provider: Arc<dyn ConnectionProvider>,
}

impl BrandModule {
  pub fn new(connection_provider: Arc<dyn ConnectionProvider>) -> Self {
    Self {
      connection_provider,
    }
  }

  pub fn resolve_register_brand_usecase(&self) -> RegisterBrandUsecase {
    let brand_repository: Arc<dyn BrandRepository> = Arc::new(BrandRepositoryOnRdbms::new(
      self.connection_provider.clone(),
    ));
    let sector_repository: Arc<dyn SectorRepository> = Arc::new(SectorRepositoryOnRdbms::new(
      self.connection_provider.clone(),
    ));
    let validator: Arc<dyn Validator<BrandRegisterInput, ValidatedBrandRegisterInput>> = Arc::new(
      BrandRegisterInputValidator::new(Arc::clone(&brand_repository), sector_repository),
    );
    let ulid_generator: Arc<dyn UlidGenerator> = Arc::new(UlidGeneratorImpl);
    let factory: Arc<dyn BrandFactory> = Arc::new(BrandFactoryImpl::new(ulid_generator));

    RegisterBrandUsecase::new(brand_repository, validator, factory)
  }

  pub fn resolve_brand_list_query(&self) -> BrandListQuery {
    BrandListQuery::new(self.connection_provider.clone())
  }
}
