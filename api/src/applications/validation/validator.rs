use axum::async_trait;
use mockall::automock;

use crate::applications::errors::application_error::ApplicationError;

#[async_trait]
#[automock]
pub trait Validator<T: std::marker::Send + std::marker::Sync>: Sync + Send {
  async fn validate(&self, target: &T) -> Result<(), ApplicationError>;
}
