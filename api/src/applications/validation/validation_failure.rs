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

macro_rules! validation_failure {
  ($field:expr, $key:expr $(, $params:expr)* $(,)?) => {
    $crate::applications::validation::validation_failure::ValidationFailure::new($field.to_string(), resource_key!($key $(, $params)*))
  };
}

pub(crate) use validation_failure;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::applications::common::resource_key::resource_key;

  #[test]
  fn validation_failure_should_able_to_merge_keys() {
    let failure1 = validation_failure!("field", "key1", "param1");
    let failure2 = validation_failure!("field", "key2", "param2");
    let merged = failure1.merge(&failure2);
    assert_eq!(merged.field, "field");
    assert_eq!(merged.keys.len(), 2);
    assert_eq!(merged.keys[0], resource_key!("key1", "param1"));
    assert_eq!(merged.keys[1], resource_key!("key2", "param2"));
  }

  #[test]
  fn validation_failure_should_display_field_and_key_and_params() {
    let failure = validation_failure!("field", "key", "param");
    assert_eq!(failure.to_string(), "field:key(param)");
  }

  #[test]
  fn validation_failure_should_allow_not_string_params() {
    let failure = validation_failure!("field", "key", 1);
    assert_eq!(failure.to_string(), "field:key(1)");
  }
}