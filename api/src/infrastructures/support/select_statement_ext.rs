use sea_orm::sea_query::SelectStatement;

pub trait SelectStatementExt {
  fn when(&mut self, predicate: bool, f: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self;
}

impl SelectStatementExt for SelectStatement {
  fn when(&mut self, predicate: bool, f: impl FnOnce(&mut Self) -> &mut Self) -> &mut Self {
    if predicate {
      f(self)
    } else {
      self
    }
  }
}
