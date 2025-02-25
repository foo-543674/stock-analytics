use sea_orm::entity::prelude::*;

use super::sector_model::Entity as SectorModel;
use crate::{
  domains::brand::brand::{Brand, BrandCode, BrandId},
  util::{unempty_string::UnemptyString, version::Version},
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "brands")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub name: String,
  pub code: String,
  pub sector_id: String,
  pub version: i32,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::sector_model::Entity",
    from = "Column::SectorId",
    to = "super::sector_model::Column::Id"
  )]
  Sector,
}

impl Related<super::sector_model::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Sector.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
  pub async fn into_domain(
    &self,
    connection: &DatabaseConnection,
  ) -> Result<Brand, sea_orm::error::DbErr> {
    let sector = self
      .find_related(SectorModel)
      .one(connection)
      .await?
      .unwrap();

    Ok(Brand {
      id: BrandId::from_string(&self.id).unwrap(),
      name: UnemptyString::new(self.name.clone()),
      code: BrandCode::new(self.code.clone()),
      version: Version::from_value(self.version as u32),
      sector: sector.into_domain(connection).await?,
    })
  }
}
