use sea_orm::DbErr;

use crate::applications::errors::repository_error::RepositoryError;

impl From<DbErr> for RepositoryError {
  fn from(error: DbErr) -> Self {
    RepositoryError::new(Box::new(error))
  }
}
