use axum::{
  body::Body,
  http::StatusCode,
  response::{IntoResponse, Json, Response},
};
use tracing::error;

use crate::applications::{
  errors::application_error::ApplicationError, validation::validation_failure::ValidationFailure,
};

impl IntoResponse for ApplicationError {
  fn into_response(self) -> axum::http::Response<axum::body::Body> {
    match self {
      ApplicationError::ValidationError(failure) => {
        let body: Json<ValidationFailure> = Json(failure);
        (StatusCode::BAD_REQUEST, body).into_response()
      }
      ApplicationError::ConflictError(_) => Response::builder()
        .status(StatusCode::CONFLICT)
        .body(Body::empty())
        .expect("Failed to build response"),
      ApplicationError::RepositoryError(err) => {
        error!("{}", err);
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Body::empty())
          .expect("Failed to build response")
      }
      ApplicationError::UnexpectedError(err) => {
        error!("{}", err);
        Response::builder()
          .status(StatusCode::INTERNAL_SERVER_ERROR)
          .body(Body::empty())
          .expect("Failed to build response")
      }
    }
  }
}
