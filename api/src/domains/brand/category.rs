use crate::{domains::macros::entity_id::*, util::unempty_string::UnemptyString};

define_id!(CategoryId);

#[derive(Debug, Clone)]
pub struct Category {
  pub id: CategoryId,
  pub name: UnemptyString,
}

impl PartialEq<Category> for Category {
  fn eq(&self, other: &Category) -> bool {
    self.id == other.id
  }
}

impl Eq for Category {}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for Category {
  fn default() -> Self {
    Category {
      id: CategoryId::new(default_ulid()),
      name: UnemptyString::from_string("category"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::test_support::unambiguous_ulid::unambiguous_ulid;

  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn category_should_not_be_equal_when_id_is_different(
      id1 in unambiguous_ulid(),
      id2 in unambiguous_ulid(),
    ) {
      let category1 = Category {
        id: CategoryId::from_string(&id1),
        name: UnemptyString::from_string("category"),
      };
      let category2 = Category {
        id: CategoryId::from_string(&id2),
        name: UnemptyString::from_string("category"),
      };
      assert_ne!(category1, category2);
    }
  }

  #[test]
  fn category_should_be_equal_when_id_is_same() {
    let category1 = Category::default();
    let category2 = Category::default();
    assert_eq!(category1, category2);
  }
}