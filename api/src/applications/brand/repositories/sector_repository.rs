use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::{
  applications::errors::repository_error::RepositoryError,
  domains::brand::sector::{Sector, SectorId},
};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait SectorRepository: Sync + Send {
  async fn find_by_id(&self, id: &SectorId) -> Result<Option<Sector>, RepositoryError>;
}
