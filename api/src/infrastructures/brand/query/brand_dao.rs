use sea_orm::{sea_query::*, *};

use crate::infrastructures::support::select_statement_ext::SelectStatementExt;

use super::{
  brand_query_request::BrandListQueryRequest,
  table_iden::{Brands, Categories, SectorGroups, Sectors},
};

#[derive(Clone, Debug, PartialEq, FromQueryResult)]
pub struct BrandRecord {
  pub id: String,
  pub name: String,
  pub code: String,
  pub version: i32,
  pub sector_id: String,
  pub sector_code: String,
  pub sector_name: String,
  pub sector_group_name: String,
  pub sector_group_code: i32,
  pub category_name: String,
}

pub fn brands_query() -> SelectStatement {
  Query::select()
    .expr_as(Expr::col((Brands::Table, Brands::Id)), Alias::new("id"))
    .expr_as(Expr::col((Brands::Table, Brands::Name)), Alias::new("name"))
    .expr_as(Expr::col((Brands::Table, Brands::Code)), Alias::new("code"))
    .expr_as(
      Expr::col((Brands::Table, Brands::SectorId)),
      Alias::new("sector_id"),
    )
    .expr_as(
      Expr::col((Brands::Table, Brands::Version)),
      Alias::new("version"),
    )
    .expr_as(
      Expr::col((Sectors::Table, Sectors::Name)),
      Alias::new("sector_name"),
    )
    .expr_as(
      Expr::col((Sectors::Table, Sectors::Code)),
      Alias::new("sector_code"),
    )
    .expr_as(
      Expr::col((SectorGroups::Table, SectorGroups::Name)),
      Alias::new("sector_group_name"),
    )
    .expr_as(
      Expr::col((SectorGroups::Table, SectorGroups::Code)),
      Alias::new("sector_group_code"),
    )
    .expr_as(
      Expr::col((Categories::Table, Categories::Name)),
      Alias::new("category_name"),
    )
    .from(Brands::Table)
    .left_join(
      Sectors::Table,
      Expr::col((Brands::Table, Brands::SectorId)).equals((Sectors::Table, Sectors::Id)),
    )
    .left_join(
      SectorGroups::Table,
      Expr::col((Sectors::Table, Sectors::SectorGroupId))
        .equals((SectorGroups::Table, SectorGroups::Id)),
    )
    .left_join(
      Categories::Table,
      Expr::col((Sectors::Table, Sectors::CategoryId)).equals((Categories::Table, Categories::Id)),
    )
    .to_owned()
}

pub struct BrandDao;

impl BrandDao {
  pub async fn find_by_query(
    request: &BrandListQueryRequest,
    db: &DatabaseConnection,
  ) -> Result<Vec<BrandRecord>, sea_orm::error::DbErr> {
    let query = brands_query()
      .limit(request.pagination.limit() as u64)
      .offset(request.pagination.offset() as u64)
      .when(!request.sorts.is_empty(), |q| {
        request.sorts.iter().fold(q, |q, sort| {
          q.order_by(sort.key.get_column_from_sort_key(), sort.order.clone())
        })
      })
      .when(request.sector_id.is_some(), |q| {
        q.and_where(
          Expr::col((Brands::Table, Brands::SectorId)).eq(request.sector_id.as_ref().unwrap()),
        )
      })
      .to_owned();

    let builder = db.get_database_backend();
    let stmt = builder.build(&query);

    return BrandRecord::find_by_statement(stmt).all(db).await;
  }
}
