use crate::{domains::macros::entity_id::*, util::unempty_string::UnemptyString};

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

  pub fn from_i32(value: i32) -> Self {
    SectorGroupCode::new(value.clone())
  }
}

impl PartialEq<SectorGroup> for SectorGroup {
  fn eq(&self, other: &SectorGroup) -> bool {
    self.id == other.id
  }
}

impl Eq for SectorGroup {}

#[derive(Debug, Clone)]
pub struct SectorGroup {
  pub id: SectorGroupId,
  pub name: UnemptyString,
  pub code: SectorGroupCode,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for SectorGroup {
  fn default() -> Self {
    SectorGroup {
      id: SectorGroupId::new(default_ulid()),
      name: UnemptyString::from_string("sector group"),
      code: SectorGroupCode::new(1),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::panic::*;
  use proptest::prelude::*;
  use crate::test_support::ulid::random_ulid;

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

    #[test]
    fn sector_group_should_not_be_equal_when_id_is_different(
      id1 in random_ulid(),
      id2 in random_ulid(),
    ) {
      let sector_group1 = SectorGroup {
        id: SectorGroupId::new(id1),
        name: UnemptyString::from_string("sector group"),
        code: SectorGroupCode::from_i32(1),
      };
      let sector_group2 = SectorGroup {
        id: SectorGroupId::new(id2),
        name: UnemptyString::from_string("sector group"),
        code: SectorGroupCode::from_i32(1),
      };
      assert_ne!(sector_group1, sector_group2);
    }
  }

  #[test]
  fn sector_group_should_be_equal_when_id_is_same() {
    let sector_group1 = SectorGroup::default();
    let sector_group2 = SectorGroup::default();
    assert_eq!(sector_group1, sector_group2);
  }
}
