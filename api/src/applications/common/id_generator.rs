#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait IdGenerator<T: Sync + Send> {
  fn generate(&self) -> T;
}
