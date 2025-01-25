use crate::queries::common::{
  pagination::Pagination, 
  sort::{sort_key, Sort, SortKey}
};

sort_key!(BrandSortKey, Id, Name, Code, SectorCode, SectorGroupCode, SectorCategory);

pub struct BrandListQueryRequest {
  pub pagination: Pagination,
  pub sorts: Vec<Sort<BrandSortKey>>,
  pub sector_id: Option<String>,
}