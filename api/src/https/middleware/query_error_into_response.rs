use axum::{
  response::{
    IntoResponse,
    Response,
  },
  body::Body,
  http::StatusCode,
};

use crate::infrastructures::errors::query_error::QueryError;

impl IntoResponse for QueryError {
  fn into_response(self) -> axum::http::Response<axum::body::Body> {
    return match self {
      QueryError::DatabaseError(_) => Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .expect("Failed to build response"),
    };
  }
}