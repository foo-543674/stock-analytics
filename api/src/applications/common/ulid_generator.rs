use ulid::Ulid;
#[cfg(test)]
use mockall::automock;

use crate::applications::errors::application_error::ApplicationError;

#[cfg_attr(test, automock)]
pub trait UlidGenerator {
  fn generate(&self) -> Result<Ulid, ApplicationError>;
}
