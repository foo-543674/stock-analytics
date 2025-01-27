use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::infrastructures::errors::query_error::QueryError;
use super::{brand_dao::{BrandDao, BrandRecord}, brand_query_request::BrandListQueryRequest};

pub struct BrandListQuery {
  connection: Arc<DatabaseConnection>,
}

impl BrandListQuery {
  pub fn new(connection: Arc<DatabaseConnection>) -> Self {
    Self { connection }
  }

  pub async fn exec(&self, request: &BrandListQueryRequest) -> Result<Vec<BrandRecord>, QueryError> {
    Ok(BrandDao::find_by_query(request, &self.connection).await?)
  }
}