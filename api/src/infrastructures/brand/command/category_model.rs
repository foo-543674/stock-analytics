use sea_orm::entity::prelude::*;

use crate::{domains::brand::category::{Category, CategoryId}, util::unempty_string::UnemptyString};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: String,
  pub name: String,
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
  pub async fn into_domain(&self) -> Result<Category, sea_orm::error::DbErr> {
    let id = CategoryId::from_string(&self.id).unwrap();
    let name = UnemptyString::from_string(&self.name);
    Ok(Category { id, name })
  }
}