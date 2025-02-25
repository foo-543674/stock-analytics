use super::brand_json::BrandJson;
use crate::infrastructures::{
  brand::query::{brand_list_query::BrandListQuery, brand_query_request::BrandListQueryRequest},
  errors::query_error::QueryError,
  query::{pagination::Pagination, sort::Sort},
};
use axum::{extract::Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBrandListQueryParameter {
  page: Option<i32>,
  count: Option<i32>,
  sort: Option<String>,
  sector: Option<String>,
  // name: Option<String>,
  // code: Option<String>
}

impl GetBrandListQueryParameter {
  fn to_query_request(&self) -> BrandListQueryRequest {
    BrandListQueryRequest {
      pagination: Pagination::from_int_option(self.page, self.count),
      sorts: self
        .sort
        .as_ref()
        .map(|v| Sort::from_string_with_canma_separated(&v))
        .unwrap_or(Default::default()),
      sector_id: self.sector.clone(),
    }
  }
}

pub async fn get_brands(
  param: Query<GetBrandListQueryParameter>,
  query: &BrandListQuery,
) -> Result<Json<Vec<BrandJson>>, QueryError> {
  let request = param.to_query_request();
  let result = query.exec(&request).await?;
  Ok(Json(
    result
      .into_iter()
      .map(BrandJson::from_brand_record)
      .collect(),
  ))
}
