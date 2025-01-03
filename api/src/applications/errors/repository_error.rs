use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RepositoryError(Box<dyn Error>);

impl fmt::Display for RepositoryError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for RepositoryError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    self.0.source()
  }
}