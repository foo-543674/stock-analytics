use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait};

use super::sector_model::Entity;
use crate::{
  applications::{
    brand::repositories::sector_repository::SectorRepository,
    errors::repository_error::RepositoryError,
  },
  domains::brand::sector::{Sector, SectorId},
  infrastructures::support::connection::ConnectionProvider,
};

pub struct SectorRepositoryOnRdbms {
  connection_provider: Arc<dyn ConnectionProvider>,
}

impl SectorRepositoryOnRdbms {
  pub fn new(connection_provider: Arc<dyn ConnectionProvider>) -> Self {
    Self {
      connection_provider,
    }
  }
}

#[async_trait]
impl SectorRepository for SectorRepositoryOnRdbms {
  async fn find_by_id(&self, id: &SectorId) -> Result<Option<Sector>, RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    let result_opt = Entity::find_by_id(id.value().to_string())
      .one(&*connection)
      .await?;

    if let Some(result) = result_opt {
      Ok(Some(result.into_domain(&*connection).await?))
    } else {
      Ok(None)
    }
  }
}
