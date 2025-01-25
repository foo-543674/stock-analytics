use sea_orm::{
  *,
  sea_query::*, 
  Order as SeaOrder
};

use crate::{
  infrastructures::support::{
    order::OrderExt,
    select_statement_ext::SelectStatementExt
  }, 
  queries::brand::brand_list_query::{
    BrandListQueryRequest, 
    BrandSortKey
  }
};

#[derive(Clone, Debug, PartialEq, FromQueryResult)]
pub struct BrandRecord {
  pub id: String,
  pub name: String,
  pub code: String,
  pub sector_id: String,
  pub version: i32,
}

#[derive(DeriveIden)]
pub enum Brands {
  Table,
  Id,
  Name,
  Code,
  SectorId,
  Version,
}

#[derive(DeriveIden)]
pub enum Sectors {
  Table,
  Id,
  Name,
  Code,
  SectorGroupId,
  CategoryId,
}

#[derive(DeriveIden)]
pub enum SectorGroups {
  Table,
  Id,
  Name,
  Code,
}

#[derive(DeriveIden)]
pub enum Categories {
  Table,
  Id,
  Name,
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum BrandRelation {
  Sector,
  SectorGroup,
  Category,
}

fn get_column_from_sort_key(key: &BrandSortKey) -> ColumnRef {
  match key {
    BrandSortKey::Id => (Brands::Table, Brands::Id).into_column_ref(),
    BrandSortKey::Name => (Brands::Table, Brands::Name).into_column_ref(),
    BrandSortKey::Code => (Brands::Table, Brands::Code).into_column_ref(),
    BrandSortKey::SectorCode => (Sectors::Table, Sectors::Code).into_column_ref(),
    BrandSortKey::SectorGroupCode => (SectorGroups::Table, SectorGroups::Code).into_column_ref(),
    BrandSortKey::SectorCategory => (Categories::Table, Categories::Name).into_column_ref(),
  }
}

pub struct BrandDao;

pub fn brands_query(request: BrandListQueryRequest) -> SelectStatement{
  Query::select()
    .columns(vec![
      (Brands::Table, Brands::Id),
      (Brands::Table, Brands::Name),
      (Brands::Table, Brands::Code),
      (Brands::Table, Brands::SectorId),
      (Brands::Table, Brands::Version),
    ])
    .columns(vec![
      (Sectors::Table, Sectors::Id),
      (Sectors::Table, Sectors::Name),
      (Sectors::Table, Sectors::Code),
      (Sectors::Table, Sectors::SectorGroupId),
      (Sectors::Table, Sectors::CategoryId),
    ])
    .columns(vec![
      (SectorGroups::Table, SectorGroups::Id),
      (SectorGroups::Table, SectorGroups::Name),
      (SectorGroups::Table, SectorGroups::Code),
    ])
    .columns(vec![
      (Categories::Table, Categories::Id),
      (Categories::Table, Categories::Name),
    ])
    .from(Brands::Table)
    .left_join(
      Sectors::Table, 
      Expr::col((Brands::Table, Brands::SectorId)).equals((Sectors::Table, Sectors::Id))
    )
    .left_join(
      SectorGroups::Table, 
      Expr::col((Sectors::Table, Sectors::SectorGroupId)).equals((SectorGroups::Table, SectorGroups::Id))
    )
    .left_join(
      Categories::Table, 
      Expr::col((Sectors::Table, Sectors::CategoryId)).equals((Categories::Table, Categories::Id))
    )
    .limit(request.pagination.limit() as u64)
    .offset(request.pagination.offset() as u64)
    .when(!request.sorts.is_empty(), |q| {
      request
        .sorts
        .iter()
        .fold(q, |q, sort| {
          q.order_by(get_column_from_sort_key(&sort.key), SeaOrder::from_order(&sort.order))
        })
    })
    .when(request.sector_id.is_some(), |q| {
      q.and_where(Expr::col((Brands::Table, Brands::SectorId)).eq(request.sector_id.as_ref().unwrap()))
    })
    .to_owned()
}

//TODO: WIP
impl BrandDao {
  async fn find_by_query(request: BrandListQueryRequest, db: DatabaseConnection) -> Result<Vec<BrandRecord>, sea_orm::error::DbErr> {
    let query = brands_query(request);

    let builder = db.get_database_backend();
    let stmt = builder.build(&query);

    return BrandRecord::find_by_statement(stmt).all(&db).await;
  }
}

#[cfg(test)]
fn normalize_multiline(text: &str) -> String {
  text.lines()
    .map(|line| line.trim())
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>()
    .join(" ")
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::queries::common::{pagination::Pagination, sort::{Order, Sort}};
  use sea_orm::sea_query::MysqlQueryBuilder;

  #[test]
  fn brands_query_should_create_query_with_pagination_and_sorts() {
    let request = BrandListQueryRequest {
      pagination: Pagination::new(2, 20),
      sorts: vec![
        Sort::new(BrandSortKey::Id, Order::Ascending),
        Sort::new(BrandSortKey::Name, Order::Descending),
      ],
      sector_id: Some("01JJF78H6MQNSCRSBB79DB1PDV".to_string()),
    };

    let query = brands_query(request);
    let query_str = query.to_string(MysqlQueryBuilder);

    let expect = "
      SELECT `id`, `name`, `code`, `sector_id`, `version` 
      FROM `brands` 
      ORDER BY `id` ASC, `name` DESC 
      LIMIT 20 
      OFFSET 20
    ";

    assert_eq!(query_str, normalize_multiline(expect));
  }
}