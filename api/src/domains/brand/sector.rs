use crate::domains::macros::define_id::*;
use crate::domains::brand::sector_group::SectorGroup;
use crate::domains::brand::category::Category;

define_id!(SectorId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SectorCode(String);

impl SectorCode {
  pub fn value(&self) -> &String {
    &self.0
  }

  const SECTOR_CODE_LENGTH: usize = 4;
  pub fn new(value: String) -> Self {
    if value.len() != Self::SECTOR_CODE_LENGTH {
      panic!("SectorCode must be {} characters", Self::SECTOR_CODE_LENGTH);
    }
    if !value.chars().all(|c| c.is_numeric()) {
      panic!("SectorCode must be numeric");
    }
    SectorCode(value)
  }
}

pub struct Sector {
  pub id: SectorId,
  pub name: String,
  pub code: SectorCode,
  pub group: SectorGroup,
  pub category: Category,
}

#[cfg(test)]
mod tests {
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
  }
}
