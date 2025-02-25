use crate::{
  https::brand::{
    brand_json::BrandJson,
    get_brands::{get_brands, GetBrandListQueryParameter},
    post_brands::{post_brands, PostBrandBody},
  },
  modules::brand::BrandModule,
};
use axum::{
  extract::{Query, State},
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use std::sync::Arc;

async fn post_handler(
  State(module): State<Arc<BrandModule>>,
  body: Json<PostBrandBody>,
) -> Result<Json<BrandJson>, impl IntoResponse> {
  let usecase = module.resolve_register_brand_usecase();
  return post_brands(body, &usecase).await;
}

async fn get_list_handler(
  State(module): State<Arc<BrandModule>>,
  param: Query<GetBrandListQueryParameter>,
) -> Result<Json<Vec<BrandJson>>, impl IntoResponse> {
  let query = module.resolve_brand_list_query();
  return get_brands(param, &query).await;
}

pub fn brand_router(module: Arc<BrandModule>) -> Router {
  let routes = Router::new()
    .route("/", post(post_handler))
    .route("/", get(get_list_handler))
    .with_state(module);

  return Router::new().nest("/brands", routes);
}
