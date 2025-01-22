pub trait SortKey: Sized + Copy + Clone + Eq + PartialEq {
  fn to_string(&self) -> &'static str;
  fn get_values() -> Vec<Self>;
  fn from_string(value: &str) -> Option<Self> {
    Self::get_values().iter().find(|&v| v.to_string().to_ascii_lowercase() == value.to_ascii_lowercase()).copied()
  }
  fn from_string_with_canma_separated(value: &str) -> Vec<Self> {
    value
      .split(',')
      .map(Self::from_string)
      .flatten()
      .collect()
  }
}

pub enum Order {
  Ascending,
  Descending,
}

const ASCENDING_KEY: &str = "asc";
const DESCENDING_KEY: &str = "desc";

impl Order {
  fn to_string(&self) -> &'static str {
    match self {
      Order::Ascending => ASCENDING_KEY,
      Order::Descending => DESCENDING_KEY,
    }
  }

  fn from_string(value: &str) -> Option<Self> {
    [ASCENDING_KEY, DESCENDING_KEY].iter().find(|&v| v.to_ascii_lowercase() == value.to_ascii_lowercase()).map(|&v| {
      match v {
        ASCENDING_KEY => Order::Ascending,
        DESCENDING_KEY => Order::Descending,
        _ => unreachable!(),
      }
    })
  }
}

#[cfg(test)]
mod test {
  use crate::test_support::{
    generic::random_pick_values_from, 
    string::{
      pick_one_with_random_case_from, 
      pick_values_with_random_case_from, 
      random_text
    }
  };

  use super::*;
  use proptest::prelude::*;

  #[derive(Debug, PartialEq, Eq, Clone, Copy)]
  enum SampleSortKey {
    Id,
    Name,
    Code,
  }

  impl SortKey for SampleSortKey {
    fn to_string(&self) -> &'static str {
      match self {
        SampleSortKey::Id => "id",
        SampleSortKey::Name => "name",
        SampleSortKey::Code => "code",
      }
    }

    fn get_values() -> Vec<Self> {
      vec![SampleSortKey::Id, SampleSortKey::Name, SampleSortKey::Code]
    }
  }

  proptest!(
    #[test]
    fn sort_key_should_able_to_parse_canma_separated_string(input in random_pick_values_from!("id", "name", "code")) {
      let input_str = input.join(",");
      let result = SampleSortKey::from_string_with_canma_separated(&input_str).iter().map(|v| SampleSortKey::to_string(&v).to_string()).collect::<Vec<_>>();
      let expected = input.iter().map(|v| v.to_ascii_lowercase()).collect::<Vec<_>>();
      prop_assert_eq!(result, expected);
    }

    #[test]
    fn sort_key_should_parse_as_case_insensitive(input in pick_values_with_random_case_from!("id", "name", "code")) {
      let input_str = input.join(",");
      let result = SampleSortKey::from_string_with_canma_separated(&input_str).iter().map(|v| SampleSortKey::to_string(&v).to_string()).collect::<Vec<_>>();
      let expected = input.iter().map(|v| v.to_ascii_lowercase()).collect::<Vec<_>>();
      prop_assert_eq!(result, expected);
    }

    #[test]
    fn sort_key_should_ignore_invalid_values(input in random_pick_values_from!("id", "name", "code"), invalid in random_text()) {
      let input_str = format!("{},{}", input.join(","), invalid);
      let result = SampleSortKey::from_string_with_canma_separated(&input_str).iter().map(|v| SampleSortKey::to_string(&v).to_string()).collect::<Vec<_>>();
      let expected = input.iter().map(|v| v.to_ascii_lowercase()).collect::<Vec<_>>();
      prop_assert_eq!(result, expected);
    }

    #[test]
    fn order_should_parse_as_case_insensitive(input in pick_one_with_random_case_from!("asc", "desc")) {
      let result = Order::from_string(&input).map(|v| Order::to_string(&v).to_string());
      prop_assert_eq!(result, Some(input.to_lowercase()));
    }
  );
}