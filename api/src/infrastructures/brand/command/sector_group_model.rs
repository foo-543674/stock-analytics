use sea_orm::entity::prelude::*;

use crate::{domains::brand::sector_group::{SectorGroup, SectorGroupCode, SectorGroupId}, util::unempty_string::UnemptyString};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sector_groups")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub name: String,
  pub code: i32,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::sector_model::Entity")]
  Sector,
}

impl Related<super::sector_model::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Sector.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
  pub async fn into_domain(&self) -> Result<SectorGroup, sea_orm::error::DbErr> {
    let id = SectorGroupId::from_string(&self.id).unwrap();
    let name = UnemptyString::from_string(&self.name);
    let code = SectorGroupCode::from_i32(self.code);
    Ok(SectorGroup { id, name, code })
  }
}