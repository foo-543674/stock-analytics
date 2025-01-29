use sea_orm::DeriveIden;

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
