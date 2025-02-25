use sea_orm::entity::prelude::*;

use crate::{
  domains::brand::sector::{Sector, SectorCode, SectorId},
  util::unempty_string::UnemptyString,
};

use super::{
  category_model::Entity as CategoryModel, sector_group_model::Entity as SectorGroupModel,
};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "sectors")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub name: String,
  pub code: String,
  pub sector_group_id: String,
  pub category_id: String,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "super::brand_model::Entity")]
  Brand,
  #[sea_orm(
    belongs_to = "super::sector_group_model::Entity",
    from = "Column::SectorGroupId",
    to = "super::sector_group_model::Column::Id"
  )]
  SectorGroup,
  #[sea_orm(
    belongs_to = "super::category_model::Entity",
    from = "Column::CategoryId",
    to = "super::category_model::Column::Id"
  )]
  Category,
}

impl Related<super::brand_model::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Brand.def()
  }
}

impl Related<super::sector_group_model::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::SectorGroup.def()
  }
}

impl Related<super::category_model::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Category.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
  pub async fn into_domain(
    &self,
    connection: &DatabaseConnection,
  ) -> Result<Sector, sea_orm::error::DbErr> {
    let sector_group = self
      .find_related(SectorGroupModel)
      .one(connection)
      .await?
      .unwrap();
    let category = self
      .find_related(CategoryModel)
      .one(connection)
      .await?
      .unwrap();
    Ok(Sector {
      id: SectorId::from_string(&self.id).unwrap(),
      name: UnemptyString::new(self.name.clone()),
      code: SectorCode::from_string(&self.code),
      group: sector_group.into_domain().await?,
      category: category.into_domain().await?,
    })
  }
}
