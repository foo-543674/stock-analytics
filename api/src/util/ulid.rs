use ulid::Ulid;

pub fn default_ulid() -> Ulid {
  Ulid::from_string("00000000000000000000000000").expect("Failed to create default ulid")
}

pub trait StringExtForUlid {
  fn is_ulid(&self) -> bool;
}

impl StringExtForUlid for str {
  fn is_ulid(&self) -> bool {
    Ulid::from_string(self).is_ok()
  }
}

#[cfg(test)]
mod tests {
  use crate::test_support::{string::random_text, ulid::random_ulid};

  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn string_is_ulid_should_return_true_when_valid_ulid(value in random_ulid()) {
      assert!(value.to_string().is_ulid(), "value should be ulid: {}", value);
    }

    #[test]
    fn string_is_ulid_should_return_false_when_invalid_ulid(value in random_text()) {
      assert!(!value.is_ulid(), "value should not be ulid: {}", value);
    }
  }
}
