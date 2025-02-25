use crate::domains::brand::sector::Sector;
use crate::domains::macros::entity_id::*;
use crate::util::unempty_string::UnemptyString;

define_id!(BrandId);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrandCode(String);

impl BrandCode {
  pub fn value(&self) -> &str {
    &self.0
  }

  pub const BRAND_CODE_LENGTH: usize = 4;
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

impl PartialEq<Brand> for Brand {
  fn eq(&self, other: &Brand) -> bool {
    self.id == other.id
  }
}

impl Eq for Brand {}

#[derive(Debug, Clone)]
pub struct Brand {
  pub id: BrandId,
  pub name: UnemptyString,
  pub code: BrandCode,
  pub sector: Sector,
  pub version: Version,
}

#[cfg(test)]
use crate::util::ulid::default_ulid;
use crate::util::version::Version;

#[cfg(test)]
impl Default for Brand {
  fn default() -> Self {
    Brand {
      id: BrandId::new(default_ulid()),
      name: UnemptyString::from_string("company"),
      code: BrandCode::new("0000".to_string()),
      sector: Default::default(),
      version: Version::new(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test_support::ulid::random_ulid;
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

    #[test]
    fn brand_should_not_be_equal_when_id_is_different(
      id1 in random_ulid(),
      id2 in random_ulid(),
    ) {
      let brand1 = Brand {
        id: BrandId::new(id1),
        name: UnemptyString::from_string("company"),
        code: BrandCode::from_string("0000"),
        sector: Default::default(),
        version: Version::new(),
      };
      let brand2 = Brand {
        id: BrandId::new(id2),
        name: UnemptyString::from_string("company"),
        code: BrandCode::from_string("0000"),
        sector: Default::default(),
        version: Version::new(),
      };
      assert_ne!(brand1, brand2);
    }
  }

  #[test]
  fn brand_should_be_equal_when_id_is_same() {
    let brand1 = Brand::default();
    let brand2 = Brand::default();
    assert_eq!(brand1, brand2);
  }
}
