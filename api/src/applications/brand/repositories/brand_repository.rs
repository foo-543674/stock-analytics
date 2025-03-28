use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::{
  applications::errors::repository_error::RepositoryError,
  domains::brand::brand::{Brand, BrandCode, BrandId},
  util::version::Version,
};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait BrandRepository: Sync + Send {
  async fn find_by_id(&self, id: &BrandId) -> Result<Option<Brand>, RepositoryError>;
  async fn find_by_code(&self, code: &BrandCode) -> Result<Option<Brand>, RepositoryError>;
  async fn add(&self, brand: &Brand) -> Result<(), RepositoryError>;
  async fn update(&self, brand: &Brand, new_version: Version) -> Result<(), RepositoryError>;
  async fn delete(&self, id: &BrandId) -> Result<(), RepositoryError>;
}
