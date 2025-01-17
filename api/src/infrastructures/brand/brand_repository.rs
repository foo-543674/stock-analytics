use std::sync::RwLock;

use axum::async_trait;

use crate::{
  applications::{
    brand::repositories::brand_repository::BrandRepository, 
    errors::repository_error::RepositoryError
  }, 
  domains::brand::brand::{
    Brand, BrandCode, BrandId
  }
};

pub struct BrandRepositoryOnMemory {
  pub brands: RwLock<Vec<Brand>>,
}

impl BrandRepositoryOnMemory {
  pub fn new() -> BrandRepositoryOnMemory {
    BrandRepositoryOnMemory {
      brands: RwLock::new(vec![]),
    }
  }
}

#[async_trait]
impl BrandRepository for BrandRepositoryOnMemory {
  async fn find_by_id(&self, id: &BrandId) -> Result<Option<Brand>, RepositoryError> {
    let brands = self.brands.read().unwrap();
    let brand = brands.iter().find(|brand| brand.id == *id);
    Ok(brand.cloned())
  }

  async fn find_by_code(&self, code: &BrandCode) -> Result<Option<Brand>, RepositoryError> {
    let brands = self.brands.read().unwrap();
    let brand = brands.iter().find(|brand| brand.code == *code);
    Ok(brand.cloned())
  }

  async fn add(&self, brand: &Brand) -> Result<(), RepositoryError> {
    self.brands.write().unwrap().push(brand.clone());
    Ok(())
  }

  async fn update(&self, brand: &Brand) -> Result<(), RepositoryError> {
    let mut brands = self.brands.write().unwrap();
    let index_opt = brands.iter().position(|b| b.id == brand.id);
    index_opt.map(|index| {
      brands[index] = brand.clone();
    });
    Ok(())
  }

  async fn delete(&self, id: &BrandId) -> Result<(), RepositoryError> {
    let mut brands = self.brands.write().unwrap();
    let index_opt = brands.iter().position(|b| b.id == *id);
    index_opt.map(|index| {
      brands.remove(index);
    });
    Ok(())
  }
}