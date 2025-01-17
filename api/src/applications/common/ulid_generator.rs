use ulid::Ulid;
#[cfg(test)]
use mockall::automock;

use crate::applications::errors::application_error::ApplicationError;

#[cfg_attr(test, automock)]
pub trait UlidGenerator: Sync + Send {
  fn generate(&self) -> Result<Ulid, ApplicationError>;
}

pub struct UlidGeneratorImpl;

impl UlidGenerator for UlidGeneratorImpl {
  fn generate(&self) -> Result<Ulid, ApplicationError> {
    Ok(Ulid::new())
  }
}