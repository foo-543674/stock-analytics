use axum::{
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Response},
};
use tracing::error;

use crate::infrastructures::errors::query_error::QueryError;

impl IntoResponse for QueryError {
  fn into_response(self) -> axum::http::Response<axum::body::Body> {
    return match self {
      QueryError::DatabaseError(err) => {
        error!("{}", err);
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Body::empty())
          .expect("Failed to build response")
      }
    };
  }
}
