use axum::async_trait;
use mockall::automock;

use crate::applications::errors::application_error::ApplicationError;

#[async_trait]
#[automock]
pub trait Validator<T> {
  async fn validate(&self, target: &T) -> Result<(), ApplicationError>;
}
