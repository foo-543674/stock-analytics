use crate::{domains::brand::brand::Brand, infrastructures::brand::query::brand_dao::BrandRecord};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct SectorJson {
  id: String,
  name: String,
  code: String,
  group: String,
  group_code: i32,
  category: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct BrandJson {
  id: String,
  name: String,
  code: String,
  sector: SectorJson,
}

impl BrandJson {
  pub fn from_brand(brand: &Brand) -> Self {
    let sector = &brand.sector;
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
        category: sector.category.name.value().to_string(),
      },
    }
  }

  pub fn from_brand_record(record: BrandRecord) -> Self {
    BrandJson {
      id: record.id,
      name: record.name,
      code: record.code,
      sector: SectorJson {
        id: record.sector_id,
        name: record.sector_name,
        code: record.sector_code,
        group: record.sector_group_name,
        group_code: record.sector_group_code,
        category: record.category_name,
      },
    }
  }
}
