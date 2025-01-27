use std::sync::Arc;
use axum::{routing::{get, post}, Router};
use crate::{https::brand::{get_brands::get_brands, post_brands::post_brands}, modules::brand::BrandModule};

pub fn brand_router(module: Arc<BrandModule>) -> Router {
  let routes = Router::new()
    .route("/", post({
      let module = Arc::clone(&module);
      move |json| {
        async move {
          post_brands(json, module.resolve_register_brand_usecase()).await
        }
      }
    }))
    .route("/", get({
      let module = Arc::clone(&module);
      let query = module.resolve_brand_list_query().await;

      move |param| {
        let module = Arc::clone(&module);
        async move {
          match module.resolve_brand_list_query().await {
            Ok(query) => get_brands(param, query).await,
            Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR.into()),
          }
        }
      }
    }));

  return Router::new()
    .nest("/brands", routes);
}