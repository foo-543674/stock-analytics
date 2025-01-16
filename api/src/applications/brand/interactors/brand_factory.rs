use std::sync::Arc;
use crate::{
  applications::{
    common::ulid_generator::UlidGenerator, 
    errors::application_error::ApplicationError
  },
  domains::brand::brand::{
    Brand, 
    BrandCode, 
    BrandId
  }, 
  util::{unempty_string::UnemptyString, version::Version}
};
use super::brand_register_input_validator::ValidatedBrandRegisterInput;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait BrandFactory {
  fn create(&self, validated_input: &ValidatedBrandRegisterInput) -> Result<Brand, ApplicationError>;
}

pub struct BrandFactoryImpl{
  id_generator: Arc<dyn UlidGenerator>,
}

impl BrandFactoryImpl {
  pub fn new(id_generator: Arc<dyn UlidGenerator>) -> Self {
    Self {
      id_generator,
    }
  }
}

impl BrandFactory for BrandFactoryImpl {
  fn create(&self, validated_input: &ValidatedBrandRegisterInput) -> Result<Brand, ApplicationError> {
    let input = &validated_input.input;
    let ulid = self.id_generator.generate()?;
    Ok(Brand{
      id: BrandId::new(ulid),
      name: UnemptyString::from_string(&input.name),
      code: BrandCode::from_string(&input.code),
      sector: validated_input.found_sector.clone(),
      version: Version::new(),
    })
  }
}

#[cfg(test)]
mod tests {

  use proptest::proptest;

  use crate::{
    applications::{
      brand::interactors::{
        brand_factory::BrandFactory, 
        brand_register_input::BrandRegisterInput, 
      },
      common::ulid_generator::MockUlidGenerator
    }, 
    domains::brand::{
      brand::BrandCode, 
      sector::Sector
    }, 
    test_support::{
      string::{
        fixed_length_numeric_string, random_text
      }, 
      ulid::random_ulid
    }
  };

  use super::*;

  proptest!{
    #[test]
    fn brand_factory_should_create_brand(
      id in random_ulid(),
      name in random_text(),
      code in fixed_length_numeric_string(BrandCode::BRAND_CODE_LENGTH),
      sector_id in random_ulid(),
    ) {
      let input = BrandRegisterInput {
        name: name.clone(),
        code: code.clone(),
        sector_id: sector_id.to_string(),
      };
      let sector: Sector = Default::default();
      let validated_input = ValidatedBrandRegisterInput {
        input: input,
        found_sector: sector.clone(),
      };

      let mut id_generator = MockUlidGenerator::new();
      let id_clone = id.clone();
      id_generator.expect_generate().returning(move || Ok(id_clone));

      let factory = BrandFactoryImpl::new(Arc::new(id_generator));
      let brand = factory.create(&validated_input).unwrap();

      assert_eq!(brand.id.value(), &id);
      assert_eq!(brand.name.value(), name);
      assert_eq!(brand.code.value(), code);
      assert_eq!(brand.sector, sector);
      assert_eq!(brand.version.value(), 0);
    }
  }
}