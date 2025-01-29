use std::sync::Arc;
use crate::infrastructures::support::connection::{create_db_options, ConnectionProviderImpl, IN_MEMORY_SQLITE};

use super::brand::BrandModule;

pub struct RootModule {
  pub brand: Arc<BrandModule>
}

impl RootModule {
  pub fn new() -> Self {
    //TODO: Get from config
    let db_option = create_db_options(IN_MEMORY_SQLITE, 5);
    let connection_provider = Arc::new(ConnectionProviderImpl::new(db_option));

    Self {
      brand: Arc::new(BrandModule::new(connection_provider))
    }
  }
}