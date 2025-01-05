use crate::domains::macros::entity_id::*;

define_id!(SectorGroupId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SectorGroupCode(i32);

impl SectorGroupCode {
  pub fn value(&self) -> i32 {
    self.0
  }

  const MIN_SECTOR_GROUP_CODE: i32 = 1;
  const MAX_SECTOR_GROUP_CODE: i32 = 17;
  pub fn new(value: i32) -> Self {
    if value < Self::MIN_SECTOR_GROUP_CODE || value > Self::MAX_SECTOR_GROUP_CODE {
      panic!(
        "SectorGroupCode must be between {} and {}",
        Self::MIN_SECTOR_GROUP_CODE, Self::MAX_SECTOR_GROUP_CODE
      );
    } else {
      SectorGroupCode(value)
    }
  }
}

pub struct SectorGroup {
  pub id: SectorGroupId,
  pub name: String,
  pub code: SectorGroupCode,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for SectorGroup {
  fn default() -> Self {
    SectorGroup {
      id: SectorGroupId::new(default_ulid()),
      name: "group".to_string(),
      code: SectorGroupCode::new(1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic::*;
  use proptest::prelude::*;

  #[test]
  fn sector_group_code_should_panic_when_below_1() {
    let result = catch_unwind(|| {
      SectorGroupCode::new(0);
    });
    assert!(result.is_err());
  }

  #[test]
  fn sector_group_code_should_panic_when_over_17() {
    let result = catch_unwind(|| {
      SectorGroupCode::new(18);
    });
    assert!(result.is_err());

  }
  proptest! {
    #[test]
    fn sector_group_code_should_create_valid_code(value in 1..=17) {
      let code = SectorGroupCode::new(value);
      assert_eq!(code.value(), value);
    }
  }
}
