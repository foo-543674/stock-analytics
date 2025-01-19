use std::sync::Arc;

use axum::{
  routing::post, Router
};

use crate::{https::brand::post_brands::post_brands, modules::root::RootModule};

pub fn router(module: RootModule) -> Router {
  let module_shared = Arc::new(module);

  let api_routes = Router::new()
    .route("/brands", post(move |json| {
      let module = module_shared.clone();
      async move {
        post_brands(json, module.brand.resolve_register_brand_usecase()).await
      }
    }));

  return Router::new()
    .nest("/api", api_routes);
}