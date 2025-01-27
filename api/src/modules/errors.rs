use sea_orm::DbErr;

pub enum ModuleError {
  DbError(DbErr),
}

impl From<DbErr> for ModuleError {
  fn from(err: DbErr) -> Self {
    ModuleError::DbError(err)
  }
}