use axum::{
  routing::post, Json, Router
};

use crate::{https::brand::post_brands::{post_brands, PostBrandBody}, modules::root::RootModule};

pub fn router(module: RootModule) -> Router {
  let api_routes = Router::new()
    .route("/brands", post(move |json: Json<PostBrandBody>| {
      let module = module.clone();
      async move {
        post_brands(json, module.brand.resolve_register_brand_usecase()).await
      }
    }));

  return Router::new()
    .nest("/api", api_routes);
}