use super::brand_model::{ActiveModel, Column, Entity};
use crate::{
  applications::{
    brand::repositories::brand_repository::BrandRepository,
    errors::repository_error::RepositoryError,
  },
  domains::brand::brand::{Brand, BrandCode, BrandId},
  infrastructures::support::connection::ConnectionProvider,
  util::version::Version,
};
use async_trait::async_trait;
use sea_orm::{
  prelude::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::sync::Arc;

pub struct BrandRepositoryOnRdbms {
  connection_provider: Arc<dyn ConnectionProvider>,
}

impl BrandRepositoryOnRdbms {
  pub fn new(connection_provider: Arc<dyn ConnectionProvider>) -> Self {
    Self {
      connection_provider,
    }
  }
}

#[async_trait]
impl BrandRepository for BrandRepositoryOnRdbms {
  async fn find_by_id(&self, id: &BrandId) -> Result<Option<Brand>, RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    let result_opt = Entity::find_by_id(id.value().to_string())
      .one(&*connection)
      .await?;

    if let Some(result) = result_opt {
      Ok(Some(result.into_domain(&*connection).await?))
    } else {
      Ok(None)
    }
  }

  async fn find_by_code(&self, code: &BrandCode) -> Result<Option<Brand>, RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    let result_opt = Entity::find()
      .filter(Column::Code.contains(code.value().to_string()))
      .one(&*connection)
      .await?;

    if let Some(result) = result_opt {
      Ok(Some(result.into_domain(&*connection).await?))
    } else {
      Ok(None)
    }
  }

  async fn add(&self, brand: &Brand) -> Result<(), RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    (ActiveModel {
      id: Set(brand.id.value().to_string()),
      name: Set(brand.name.value().to_string()),
      code: Set(brand.code.value().to_string()),
      sector_id: Set(brand.sector.id.value().to_string()),
      version: Set(brand.version.value() as i32),
    })
    .insert(&*connection)
    .await?;

    Ok(())
  }

  async fn update(&self, brand: &Brand, new_verison: Version) -> Result<(), RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    Entity::update_many()
      .col_expr(Column::Name, Expr::value(brand.name.value().to_string()))
      .col_expr(Column::Code, Expr::value(brand.code.value().to_string()))
      .col_expr(
        Column::SectorId,
        Expr::value(brand.sector.id.value().to_string()),
      )
      .col_expr(Column::Version, Expr::value(new_verison.value() as i32))
      .filter(Column::Id.eq(brand.id.value().to_string()))
      .filter(Column::Version.eq(brand.version.value() as i32))
      .exec(&*connection)
      .await?;

    Ok(())
  }

  async fn delete(&self, id: &BrandId) -> Result<(), RepositoryError> {
    let connection: Arc<DatabaseConnection> = self.connection_provider.provide().await?;

    Entity::delete_by_id(id.value().to_string())
      .exec(&*connection)
      .await?;

    Ok(())
  }
}
