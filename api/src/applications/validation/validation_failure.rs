use crate::applications::common::resource_key::ResourceKey;

#[derive(Debug, Clone)]
pub struct ValidationFailure {
  pub field: String,
  pub keys: Vec<ResourceKey>
}

impl ValidationFailure {
  pub fn new(field: String, key: ResourceKey) -> Self {
    ValidationFailure {
      field,
      keys: vec![key]
    }
  }

  pub fn merge(&self, other: &ValidationFailure) -> ValidationFailure {
    ValidationFailure {
      field: self.field.clone(),
      keys: self.keys.iter().chain(other.keys.iter()).cloned().collect()
    }
  }
}

impl std::fmt::Display for ValidationFailure{
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}:{}", self.field, self.keys.iter().map(|key| key.to_string()).collect::<Vec<_>>().join(","))
  }
}
