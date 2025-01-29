use sea_orm::sea_query::{ColumnRef, IntoColumnRef};
use crate::infrastructures::query::{pagination::Pagination, sort::{sort_key, Sort}};
use super::table_iden::{Brands, Categories, SectorGroups, Sectors};

sort_key!(BrandSortKey, Id, Name, Code, SectorCode, SectorGroupCode, SectorCategory);

impl BrandSortKey {
  pub fn get_column_from_sort_key(&self) -> ColumnRef {
    match self {
      BrandSortKey::Id => (Brands::Table, Brands::Id).into_column_ref(),
      BrandSortKey::Name => (Brands::Table, Brands::Name).into_column_ref(),
      BrandSortKey::Code => (Brands::Table, Brands::Code).into_column_ref(),
      BrandSortKey::SectorCode => (Sectors::Table, Sectors::Code).into_column_ref(),
      BrandSortKey::SectorGroupCode => (SectorGroups::Table, SectorGroups::Code).into_column_ref(),
      BrandSortKey::SectorCategory => (Categories::Table, Categories::Name).into_column_ref(),
    }
  }
}

pub struct BrandListQueryRequest {
  pub pagination: Pagination,
  pub sorts: Vec<Sort<BrandSortKey>>,
  pub sector_id: Option<String>,
}