use crate::domains::macros::entity_id::*;

define_id!(CategoryId);

#[derive(Debug, Clone)]
pub struct Category {
  pub id: CategoryId,
  pub name: String,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for Category {
  fn default() -> Self {
    Category {
      id: CategoryId::new(default_ulid()),
      name: "category".to_string(),
    }
  }
}