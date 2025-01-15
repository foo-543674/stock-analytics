use axum::{
  routing::post,
  Router
};

use crate::https::brand::post_brand_handler::PostBrandHandler;

fn api_base_path(path: &str) -> String {
  format!("/api{}", path)
}

pub fn router() -> Router {
  Router::new()
    // .route(&api_base_path(PostBrandHandler::PATH), post(PostBrandHandler::new().handle))
}