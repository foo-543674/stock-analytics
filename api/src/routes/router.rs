use std::sync::Arc;
use tower_http::trace::TraceLayer;
use axum::Router;
use crate::modules::root::RootModule;

use super::brand::brand_router;

pub fn router(module: RootModule) -> Router {
  let module_shared = Arc::new(module);

  let api_routes = Router::new()
    .merge(brand_router(module_shared.brand.clone()));

  return Router::new()
    .nest("/api", api_routes)
    .layer(TraceLayer::new_for_http());
}