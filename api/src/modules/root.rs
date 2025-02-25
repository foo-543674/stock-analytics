use crate::infrastructures::support::connection::{
  create_db_options, ConnectionProviderImpl, IN_MEMORY_SQLITE,
};
use std::sync::Arc;

use super::brand::BrandModule;

pub struct RootModule {
  pub brand: Arc<BrandModule>,
}

impl RootModule {
  pub fn new() -> Self {
    //TODO: Get from config
    let db_option = create_db_options("sqlite:./data/data.sqlite3", 5);
    let connection_provider = Arc::new(ConnectionProviderImpl::new(db_option));

    Self {
      brand: Arc::new(BrandModule::new(connection_provider)),
    }
  }
}
