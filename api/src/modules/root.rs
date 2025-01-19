use super::brand::BrandModule;

pub struct RootModule {
  pub brand: BrandModule
}

impl RootModule {
  pub fn new() -> Self {
    Self {
      brand: BrandModule::new()
    }
  }
}