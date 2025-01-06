use crate::util::unempty_string::UnemptyString;
use crate::domains::macros::entity_id::*;
use crate::domains::brand::sector_group::SectorGroup;
use crate::domains::brand::category::Category;

define_id!(SectorId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SectorCode(String);

impl SectorCode {
  pub fn value(&self) -> &str {
    &self.0
  }

  pub const SECTOR_CODE_LENGTH: usize = 4;
  pub fn new(value: String) -> Self {
    if value.len() != Self::SECTOR_CODE_LENGTH {
      panic!("SectorCode must be {} characters", Self::SECTOR_CODE_LENGTH);
    }
    if !value.chars().all(|c| c.is_numeric()) {
      panic!("SectorCode must be numeric");
    }
    SectorCode(value)
  }

  pub fn from_string(value: &str) -> Self {
    SectorCode::new(value.to_string())
  }
}

impl PartialEq<Sector> for Sector {
  fn eq(&self, other: &Sector) -> bool {
    self.id == other.id
  }
}

impl Eq for Sector {}

#[derive(Debug, Clone)]
pub struct Sector {
  pub id: SectorId,
  pub name: UnemptyString,
  pub code: SectorCode,
  pub group: SectorGroup,
  pub category: Category,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for Sector {
  fn default() -> Self {
    Sector {
      id: SectorId::new(default_ulid()),
      name: UnemptyString::from_string("sector"),
      code: SectorCode::new("0000".to_string()),
      group: Default::default(),
      category: Default::default(),
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
    fn sector_code_should_construct_with_4_numeric_characters(code in "[0-9]{4}") {
      let sector_code = SectorCode::new(code.clone());
      assert_eq!(sector_code.0, code);
    }

    #[test]
    fn sector_code_should_ensure_length_is_4(code in "[0-9]{0,3}|[0-9]{5,}") {
      let result = std::panic::catch_unwind(|| SectorCode::new(code));
      assert!(result.is_err());
    }

    #[test]
    fn sector_code_should_ensure_all_character_are_numeric(code in "[a-zA-Z0-9]{4}") {
      if !code.chars().all(|c| c.is_numeric()) {
        let result = std::panic::catch_unwind(|| SectorCode::new(code));
        assert!(result.is_err());
      }
    }

    #[test]
    fn sector_should_not_be_equal_when_id_is_different(id1 in unambiguous_ulid(), id2 in unambiguous_ulid()) {
      let sector1 = Sector {
        id: SectorId::from_string(&id1),
        name: UnemptyString::from_string("sector"),
        code: SectorCode::new("0000".to_string()),
        group: Default::default(),
        category: Default::default(),
      };
      let sector2 = Sector {
        id: SectorId::from_string(&id2),
        name: UnemptyString::from_string("sector"),
        code: SectorCode::new("0000".to_string()),
        group: Default::default(),
        category: Default::default(),
      };
      assert_ne!(sector1, sector2);
    }
  }

  #[test]
  fn sector_should_be_equal_when_id_is_equal() {
    let sector1 = Sector::default();
    let sector2 = Sector::default();
    assert_eq!(sector1, sector2);
  }
}
