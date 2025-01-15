use serde::Serialize;
use crate::domains::brand::brand::Brand;

#[derive(Clone, Debug, Serialize)]
pub struct SectorJson {
  id: String,
  name: String,
  code: String,
  group: String,
  group_code: i32,
  category: String
}

#[derive(Clone, Debug, Serialize)]
pub struct BrandJson {
  id: String,
  name: String,
  code: String,
  sector: SectorJson
}

impl BrandJson {
  pub fn from_brand(brand: Brand) -> Self {
    let sector = brand.sector;
    BrandJson {
      id: brand.id.to_string(),
      name: brand.name.value().to_string(),
      code: brand.code.value().to_string(),
      sector: SectorJson {
        id: sector.id.to_string(),
        name: sector.name.value().to_string(),
        code: sector.code.value().to_string(),
        group: sector.group.name.value().to_string(),
        group_code: sector.group.code.value(),
        category: sector.category.name.value().to_string()
      }
    }
  }
}
