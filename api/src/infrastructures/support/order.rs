use sea_orm::Order as SeaOrder;
use crate::queries::common::sort::Order;

pub trait OrderExt {
  fn from_order(order: &Order) -> Self;
}

impl OrderExt for SeaOrder {
  fn from_order(order: &Order) -> Self {
    match order {
      Order::Ascending => SeaOrder::Asc,
      Order::Descending => SeaOrder::Desc,
    }
  }
}