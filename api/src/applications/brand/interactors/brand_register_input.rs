#[derive(Debug, Clone)]
pub struct BrandRegisterInput {
  pub name: String,
  pub code: String,
  pub sector_id: String,
}

impl BrandRegisterInput {
  pub fn new(name: &str, code: &str, sector_id: &str) -> Self {
    Self {
      name: name.to_string(),
      code: code.to_string(),
      sector_id: sector_id.to_string(),
    }
  }
}