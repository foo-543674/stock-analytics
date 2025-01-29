use once_cell::sync::Lazy;
use sea_orm::Order;

pub trait SortKey
  : Sized 
  + Copy 
  + Clone 
  + Eq 
  + PartialEq 
  + strum::IntoEnumIterator
  + std::str::FromStr<Err = strum::ParseError>
  + std::fmt::Display
{}

pub struct Sort<T: SortKey> {
  pub key: T,
  pub order: Order,
}

macro_rules! sort_key {
  ($name: ident, $($var:ident),+ $(,)?) => {
    #[derive(Debug, PartialEq, Eq, Clone, Copy, strum::EnumIter, strum::EnumString, strum::Display)]
    #[strum(serialize_all = "snake_case", ascii_case_insensitive)]
    pub enum $name {
      $($var,)+
    }

    impl crate::infrastructures::query::sort::SortKey for $name { }
  };
}

pub(crate) use sort_key;

const KEY_PATTERN: Lazy<regex::Regex> = Lazy::new(|| {
  regex::Regex::new(r"^(?<desc>-?)(?<key>[a-zA-Z0-9_]+)$").expect("Invalid regex pattern")
});

impl<T: SortKey> Sort<T> {
  pub fn new(key: T, order: Order) -> Self {
    Sort { key, order }
  }

  pub fn from_string(value: &str) -> Option<Self> {
    let Some(capture) = KEY_PATTERN.captures(value) else {
      return None;
    };

    let key = match T::from_str(&capture["key"]) {
      Ok(v) => v,
      Err(_) => return None,
    };
    let order = match &capture["desc"] {
      "" => Order::Asc,
      "-" => Order::Desc,
      _ => return None,
    };

    Some(Sort::new(key, order))
  }

  pub fn from_string_with_canma_separated(value: &str) -> Vec<Self> {
    value.split(",")
      .filter_map(|v| Self::from_string(v))
      .collect()
  }
}

#[cfg(test)]
mod test {
  use crate::test_support::{
    generic::random_pick_values_from, 
    string::{
      pick_values_with_random_case_from, 
      random_text
    }
  };

  use super::*;
  use proptest::prelude::*;

  sort_key!(SampleSortKey, Id, Name, FooCode);

  proptest!(
    #[test]
    fn sort_should_able_to_parse_canma_separated_string(input in random_pick_values_from!("id", "name", "foo_code")) {
      let input_str = input.join(",");
      let result = Sort::<SampleSortKey>::from_string_with_canma_separated(&input_str);
      for (i, v) in input.iter().enumerate() {
        prop_assert_eq!(&result[i].key.to_string(), v);
        prop_assert_eq!(&result[i].order, &Order::Asc);
      }
    }

    #[test]
    fn sort_should_parse_as_case_insensitive(input in pick_values_with_random_case_from!("id", "name", "foo_code")) {
      let input_str = input.join(",");
      let result = Sort::<SampleSortKey>::from_string_with_canma_separated(&input_str);
      for (i, v) in input.iter().enumerate() {
        prop_assert_eq!(&result[i].key.to_string(), &v.to_ascii_lowercase());
        prop_assert_eq!(&result[i].order, &Order::Asc);
      }
    }

    #[test]
    fn sort_should_ignore_invalid_values(input in random_pick_values_from!("id", "name", "foo_code"), invalid in random_text()) {
      let input_str = format!("{},{}", input.join(","), invalid);
      let result = Sort::<SampleSortKey>::from_string_with_canma_separated(&input_str);
      prop_assert_eq!(result.len(), input.len());
      for (i, v) in input.iter().enumerate() {
        prop_assert_eq!(&result[i].key.to_string(), v);
        prop_assert_eq!(&result[i].order, &Order::Asc);
      }
    }

    #[test]
    fn sort_should_parse_has_hyphen_as_descending(input in random_pick_values_from!("id", "name", "foo_code")) {
      let input_str = input.iter().map(|v| format!("-{}", v)).collect::<Vec<String>>().join(",");
      let result = Sort::<SampleSortKey>::from_string_with_canma_separated(&input_str);
      for (i, v) in input.iter().enumerate() {
        prop_assert_eq!(&result[i].key.to_string(), v);
        prop_assert_eq!(&result[i].order, &Order::Desc);
      }
    }
  );
}