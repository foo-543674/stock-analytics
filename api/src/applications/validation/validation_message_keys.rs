use crate::applications::common::resource_key::{ResourceKey, resource_key};

const VALIDATION_MESSAGE_PREFIX: &str = "validation";

pub fn required() -> ResourceKey {
  resource_key!(format!("{}.required", VALIDATION_MESSAGE_PREFIX))
}
pub fn duplicate() -> ResourceKey {
  resource_key!(format!("{}.duplicate", VALIDATION_MESSAGE_PREFIX))
}
pub fn length_equals(length: usize) -> ResourceKey {
  resource_key!(format!("{}.length_equals", VALIDATION_MESSAGE_PREFIX), length.to_string())
}
pub fn numeric_only() -> ResourceKey {
  resource_key!(format!("{}.numeric_only", VALIDATION_MESSAGE_PREFIX))
}
pub fn ulid() -> ResourceKey {
  resource_key!(format!("{}.ulid", VALIDATION_MESSAGE_PREFIX))
}
pub fn resource_not_found() -> ResourceKey {
  resource_key!(format!("{}.resource_not_found", VALIDATION_MESSAGE_PREFIX))
}