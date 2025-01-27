use std::sync::Arc;
use crate::infrastructures::support::connection::{create_db_options, IN_MEMORY_SQLITE};

use super::brand::BrandModule;

pub struct RootModule {
  pub brand: Arc<BrandModule>
}

impl RootModule {
  pub fn new() -> Self {
    //TODO: Get from config
    let db_option = Arc::new(create_db_options(IN_MEMORY_SQLITE, 5));

    Self {
      brand: Arc::new(BrandModule::new(db_option))
    }
  }
}