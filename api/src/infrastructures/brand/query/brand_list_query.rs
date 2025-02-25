use super::{
  brand_dao::{BrandDao, BrandRecord},
  brand_query_request::BrandListQueryRequest,
};
use crate::infrastructures::{
  errors::query_error::QueryError, support::connection::ConnectionProvider,
};
use std::sync::Arc;

pub struct BrandListQuery {
  connection_provider: Arc<dyn ConnectionProvider>,
}

impl BrandListQuery {
  pub fn new(connection_provider: Arc<dyn ConnectionProvider>) -> Self {
    Self {
      connection_provider,
    }
  }

  pub async fn exec(
    &self,
    request: &BrandListQueryRequest,
  ) -> Result<Vec<BrandRecord>, QueryError> {
    let connection = self.connection_provider.provide().await?;
    Ok(BrandDao::find_by_query(request, &connection).await?)
  }
}
