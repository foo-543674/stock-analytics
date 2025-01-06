use crate::{domains::macros::entity_id::*, util::unempty_string::UnemptyString};

define_id!(CategoryId);

#[derive(Debug, Clone)]
pub struct Category {
  pub id: CategoryId,
  pub name: UnemptyString,
}

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