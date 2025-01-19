use std::sync::Arc;

use super::brand::BrandModule;

pub struct RootModule {
  pub brand: Arc<BrandModule>
}

impl RootModule {
  pub fn new() -> Self {
    Self {
      brand: Arc::new(BrandModule::new())
    }
  }
}