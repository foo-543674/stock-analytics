use std::sync::RwLock;

use axum::async_trait;
use ulid::Ulid;

use crate::{
  applications::{
    brand::repositories::sector_repository::SectorRepository, 
    errors::repository_error::RepositoryError
  }, 
  domains::brand::{category::{Category, CategoryId}, sector::{
    Sector, SectorCode, SectorId
  }, sector_group::{SectorGroup, SectorGroupCode, SectorGroupId}}, util::unempty_string::UnemptyString
};

pub struct SectorRepositoryOnMemory {
  sectors: RwLock<Vec<Sector>>,
}

impl SectorRepositoryOnMemory {
  pub fn new() -> SectorRepositoryOnMemory {
    SectorRepositoryOnMemory {
      sectors: RwLock::new(vec![
        Sector {
          id: SectorId::new(Ulid::from_string("01JHTK7PGE9M2KX2DQHBM8MKFB").unwrap()),
          name: UnemptyString::new("Sector 1".to_string()),
          code: SectorCode::new("SECTOR1".to_string()),
          group: SectorGroup {
            id: SectorGroupId::new(Ulid::from_string("01JHTK8ZWZP0RPVX29NANDGKFA").unwrap()),
            name: UnemptyString::new("Group 1".to_string()),
            code: SectorGroupCode::new(1),
          },
          category: Category {
            id: CategoryId::new(Ulid::from_string("01JHTK97GXA1A6QPEAFG3XF8GZ").unwrap()),
            name: UnemptyString::new("Category 1".to_string()),
          },
        },
      ]),
    }
  }
}

#[async_trait]
impl SectorRepository for SectorRepositoryOnMemory {
  async fn find_by_id(&self, id: &SectorId) -> Result<Option<Sector>, RepositoryError> {
    let sectors = self.sectors.read().unwrap();
    let sector = sectors.iter().find(|sector| sector.id == *id);
    Ok(sector.cloned())
  }
}