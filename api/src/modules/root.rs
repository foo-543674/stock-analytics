use super::brand::BrandModule;

//TODO: handle without clone
#[derive(Clone)]
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