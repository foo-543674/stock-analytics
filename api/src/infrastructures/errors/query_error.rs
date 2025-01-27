use sea_orm::DbErr;

pub enum QueryError {
  DatabaseError(DbErr),
}

impl From<DbErr> for QueryError {
  fn from(err: DbErr) -> Self {
    QueryError::DatabaseError(err)
  }
}