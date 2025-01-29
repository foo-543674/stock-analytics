#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Version(u32);

impl Version {
  pub fn new() -> Self {
    Version(0)
  }

  pub fn from_value(value: u32) -> Self {
    Version(value)
  }

  pub fn value(&self) -> u32 {
    self.0
  }

  pub fn increment(&self) -> Version {
    Version(self.0 + 1)
  }
}