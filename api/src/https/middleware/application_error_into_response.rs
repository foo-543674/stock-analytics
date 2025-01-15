use axum::{
  response::{
    IntoResponse,
    Response,
    Json
  },
  body::Body,
  http::StatusCode,
};

use crate::applications::{
  errors::application_error::ApplicationError,
  validation::validation_failure::ValidationFailure
};

impl IntoResponse for ApplicationError {
  fn into_response(self) -> axum::http::Response<axum::body::Body> {
    return match self {
      ApplicationError::ValidationError(failure) => {
        let body: Json<ValidationFailure> = Json(failure);
        (StatusCode::BAD_REQUEST, body).into_response()
      },
      ApplicationError::ConflictError(_) => Response::builder()
        .status(StatusCode::CONFLICT)
        .body(Body::empty())
        .unwrap(),
      ApplicationError::RepositoryError(_) => Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap(),
      ApplicationError::UnexpectedError(_) => Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap(),
    };
  }
}