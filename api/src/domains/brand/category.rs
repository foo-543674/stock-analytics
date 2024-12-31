use crate::domains::macros::define_id::*;

define_id!(CategoryId);

pub struct Category {
  pub id: CategoryId,
  pub name: String,
}