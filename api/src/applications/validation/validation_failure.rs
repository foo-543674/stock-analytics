use crate::applications::common::resource_key::ResourceKey;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationFailedField {
  pub name: String,
  pub keys: Vec<ResourceKey>,
}

impl ValidationFailedField {
  pub fn new(name: String, key: ResourceKey) -> Self {
    ValidationFailedField {
      name,
      keys: vec![key],
    }
  }

  pub fn merge(&self, other: &ValidationFailedField) -> ValidationFailedField {
    ValidationFailedField {
      name: self.name.clone(),
      keys: self.keys.iter().chain(other.keys.iter()).cloned().collect(),
    }
  }
}

impl std::fmt::Display for ValidationFailedField {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}:{}",
      self.name,
      self
        .keys
        .iter()
        .map(|key| key.to_string())
        .collect::<Vec<_>>()
        .join(",")
    )
  }
}

macro_rules! validation_failure_field {
  ($field:expr, $key:expr) => {
    $crate::applications::validation::validation_failure::ValidationFailedField::new(
      $field.to_string(),
      $key,
    )
  };
}

pub(crate) use validation_failure_field;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationFailure {
  pub fields: Vec<ValidationFailedField>,
}

impl ValidationFailure {
  pub fn new(fields: Vec<ValidationFailedField>) -> Self {
    ValidationFailure { fields }
  }

  pub fn merge(&self, other: &ValidationFailure) -> ValidationFailure {
    ValidationFailure {
      fields: self
        .fields
        .iter()
        .fold(
          self.fields.clone(),
          |acc: Vec<ValidationFailedField>, field| {
            let same_name_fields: Vec<_> = other
              .fields
              .iter()
              .filter(|f| f.name == field.name)
              .collect();
            if same_name_fields.is_empty() {
              acc
            } else {
              let merged = same_name_fields
                .iter()
                .fold(field.clone(), |acc: ValidationFailedField, f| acc.merge(f));
              acc
                .into_iter()
                .filter(|f| f.name != merged.name)
                .chain(vec![merged.clone()])
                .collect()
            }
          },
        )
        .into_iter()
        .chain(
          other
            .fields
            .iter()
            .filter(|f| !self.has_field(&f.name))
            .cloned(),
        )
        .collect(),
    }
  }

  pub fn has_field(&self, field_name: &str) -> bool {
    self.fields.iter().any(|field| field.name == field_name)
  }

  pub fn fields_count(&self) -> usize {
    self.fields.len()
  }
}

impl std::fmt::Display for ValidationFailure {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "validation failure: {}",
      self
        .fields
        .iter()
        .map(|field| field.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    )
  }
}

macro_rules! validation_failure {
  ($field:literal, $($keys:expr),+) => {
    $crate::applications::validation::validation_failure::ValidationFailure::new(
      vec![$(crate::applications::validation::validation_failure::validation_failure_field!($field, $keys)),+]
    )
  };

  ($field:expr, $($keys:expr),+) => {
    $crate::applications::validation::validation_failure::ValidationFailure::new(
      vec![$(crate::applications::validation::validation_failure::validation_failure_field!($field, $keys)),+]
    )
  };

  (@list $($fields:expr),+) => {{
    let merged = vec![$($fields),+].into_iter().fold(Vec::<ValidationFailedField>::new(), |mut acc, field| {
      let existing = acc.iter().position(|f| f.name == field.name);
      match existing {
        Some(index) => {
          acc[index] = acc[index].merge(&field);
        },
        None => {
          acc.push(field);
        }
      }
      acc
    });
    $crate::applications::validation::validation_failure::ValidationFailure::new(merged)
  }};
}

pub(crate) use validation_failure;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::applications::validation::validation_message_keys::{
    duplicate, length_equals, required,
  };

  #[test]
  fn validation_failed_field_should_display_field_name_and_code() {
    let field = validation_failure_field!("name", required());
    assert_eq!(field.to_string(), "name:validation.required()");
  }

  #[test]
  fn validation_failed_field_should_display_field_name_and_code_with_params() {
    let field = validation_failure_field!("name", length_equals(10));
    assert_eq!(field.to_string(), "name:validation.length_equals(10)");
  }

  #[test]
  fn validation_failed_field_should_able_to_merge_keys() {
    let field1 = validation_failure_field!("name", required());
    let field2 = validation_failure_field!("name", length_equals(10));
    let merged = field1.merge(&field2);
    assert_eq!(
      merged.to_string(),
      "name:validation.required(),validation.length_equals(10)"
    );
  }

  #[test]
  fn validation_failure_should_display_fields() {
    let field1 = validation_failure_field!("name", required());
    let field2 = validation_failure_field!("name", length_equals(10));
    let failure = validation_failure!(@list field1, field2);
    assert_eq!(
      failure.to_string(),
      "validation failure: name:validation.required(),validation.length_equals(10)"
    );
  }

  #[test]
  fn validation_failure_should_able_to_merge_fields() {
    let field1 = validation_failure_field!("name", required());
    let field2 = validation_failure_field!("name", length_equals(10));
    let field3 = validation_failure_field!("code", required());
    let field4 = validation_failure_field!("code", duplicate());
    let failure = validation_failure!(@list field1);
    let merged = failure.merge(&validation_failure!(@list field2, field3, field4));
    assert_eq!(merged.to_string(), "validation failure: name:validation.required(),validation.length_equals(10)\ncode:validation.required(),validation.duplicate()");
    assert_eq!(merged.fields_count(), 2);
  }

  #[test]
  fn validation_failure_should_able_to_check_field() {
    let field1 = validation_failure_field!("name", required());
    let field2 = validation_failure_field!("name", length_equals(10));
    let failure = validation_failure!(@list field1, field2);
    assert!(failure.has_field("name"));
    assert!(!failure.has_field("code"));
  }
}
