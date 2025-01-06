#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnemptyString(String);

impl UnemptyString {
  pub fn new(value: String) -> Self {
    if value.is_empty() {
      panic!("UnemptyString should not be empty");
    }
    UnemptyString(value)
  }

  pub fn from_string(value: &str) -> Self {
    UnemptyString::new(value.to_string())
  }

  pub fn value(&self) -> &str {
    &self.0
  }
}

impl From<UnemptyString> for String {
  fn from(value: UnemptyString) -> Self {
    value.0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_support::string::random_text;
  use std::panic::*;
  use proptest::prelude::*;

  #[test]
  fn unempty_string_should_panic_when_empty() {
    let result = catch_unwind(|| {
      UnemptyString::from_string("");
    });
    assert!(result.is_err());
  }

  proptest! {
    #[test]
    fn unempty_string_should_construct_with_non_empty_string(value in random_text()) {
      let unempty_string = UnemptyString::from_string(&value);
      assert_eq!(unempty_string.value(), value);
    }
  }
}