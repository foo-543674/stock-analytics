use std::sync::Arc;
use axum::{routing::post, Router};
use crate::{https::brand::post_brands::post_brands, modules::brand::BrandModule};

pub fn brand_router(module: Arc<BrandModule>) -> Router {
  let routes = Router::new()
    .route("/", post(move |json| {
      async move {
        post_brands(json, module.resolve_register_brand_usecase()).await
      }
    }));

  return Router::new()
    .nest("/brands", routes);
}