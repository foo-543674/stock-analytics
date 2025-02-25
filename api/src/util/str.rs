pub trait StringExt {
  fn is_numeric(&self) -> bool;
}

impl StringExt for str {
  fn is_numeric(&self) -> bool {
    !self.is_empty() && self.chars().all(|c| c.is_numeric())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn string_is_numeric_should_return_true_when_numeric(value in "[0-9]+") {
      assert_eq!(value.is_numeric(), true, "value should be numeric: {}", value);
    }

    #[test]
    fn string_is_numeric_should_return_false_when_not_numeric(value in "[^0-9]*") {
      assert_eq!(value.is_numeric(), false, "value should not be numeric: {}", value);
    }
  }
}
