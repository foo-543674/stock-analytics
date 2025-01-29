use sea_orm::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueryError {
  #[error("Database error: {0}")]
  DatabaseError(#[from] DbErr),
}
