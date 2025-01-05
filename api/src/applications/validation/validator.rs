use axum::async_trait;
#[cfg(test)]
use mockall::automock;

use crate::applications::errors::application_error::ApplicationError;

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Validator<T: std::marker::Send + std::marker::Sync>: Sync + Send {
  async fn validate(&self, target: &T) -> Result<(), ApplicationError>;
}
