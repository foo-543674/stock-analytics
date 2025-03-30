use crate::{config::app_config::AppConfig, infrastructures::support::connection::{create_db_options, ConnectionProviderImpl}};
use std::sync::Arc;

use super::brand::BrandModule;

pub struct RootModule {
  pub brand: Arc<BrandModule>,
}

impl RootModule {
  pub fn new(config: &AppConfig) -> Self {
    let db_option = create_db_options(&config.database.url, config.database.pool_size);
    let connection_provider = Arc::new(ConnectionProviderImpl::new(db_option));

    Self {
      brand: Arc::new(BrandModule::new(connection_provider)),
    }
  }
}
