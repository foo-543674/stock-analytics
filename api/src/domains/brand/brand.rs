use crate::domains::macros::entity_id::*;
use crate::domains::brand::sector::Sector;

define_id!(BrandId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrandCode(String);

impl BrandCode {
  pub fn value(&self) -> &String {
    &self.0
  }

  const BRAND_CODE_LENGTH: usize = 4;
  pub fn new(value: String) -> Self {
    if value.len() != Self::BRAND_CODE_LENGTH {
      panic!("BrandCode must be {} characters", Self::BRAND_CODE_LENGTH);
    }
    if !value.chars().all(|c| c.is_numeric()) {
      panic!("SectorCode must be numeric");
    }
    BrandCode(value)
  }

  pub fn from_string(value: &str) -> Self {
    BrandCode::new(value.to_string())
  }
}

pub struct Brand {
  pub id: BrandId,
  pub name: String,
  pub code: BrandCode,
  pub sector: Sector,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;

#[cfg(test)]
impl Default for Brand {
  fn default() -> Self {
    Brand {
      id: BrandId::new(default_ulid()),
      name: "company".to_string(),
      code: BrandCode::new("0000".to_string()),
      sector: Default::default(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn brand_code_should_construct_with_4_numeric_characters(code in "[0-9]{4}") {
      let brand_code = BrandCode::new(code.clone());
      assert_eq!(brand_code.0, code);
    }

    #[test]
    fn brand_code_should_ensure_length_is_4(code in "[0-9]{0,3}|[0-9]{5,}") {
      let result = std::panic::catch_unwind(|| BrandCode::new(code));
      assert!(result.is_err());
    }

    #[test]
    fn brand_code_should_ensure_all_character_are_numeric(code in "[a-zA-Z0-9]{4}") {
      if !code.chars().all(|c| c.is_numeric()) {
        let result = std::panic::catch_unwind(|| BrandCode::new(code));
        assert!(result.is_err());
      }
    }
  }
}