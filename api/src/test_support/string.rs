#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use proptest::strategy::Strategy;
#[cfg(test)]
use super::generic::random_pick_values;

#[cfg(test)]
pub fn alphanumeric_string(length: usize) -> impl Strategy<Value = String> {
  let pattern = format!("[a-zA-Z0-9]{{{}}}", length);
  proptest::string::string_regex(&pattern).unwrap()
    .prop_map(|s: String| {
      let mut chars: Vec<char> = s.chars().collect();
      if chars.iter().all(|c| c.is_numeric()) {
        chars[0] = 'a';
      }
      chars.into_iter().collect()
    })
}

#[cfg(test)]
pub fn fixed_length_numeric_string(length: usize) -> impl Strategy<Value = String> {
  let pattern = format!("[0-9]{{{}}}", length);
  proptest::string::string_regex(&pattern).unwrap()
}

#[cfg(test)]
pub fn fixed_length_numeric_string_except(length: usize) -> impl Strategy<Value = String> {
  let pattern = format!("[0-9]{{0,{}}}|[0-9]{{{},}}", length - 1, length + 1);
  proptest::string::string_regex(&pattern).unwrap()
}

#[cfg(test)]
pub fn empty() -> impl Strategy<Value = String> {
  Just("".to_string())
}

#[cfg(test)]
pub fn random_text() -> impl Strategy<Value = String> {
  proptest::string::string_regex("\\PC+").unwrap()
}

#[cfg(test)]
pub fn random_text_length_at_most(max_length: usize) -> impl Strategy<Value = String> {
  proptest::collection::vec(any::<char>(), 1..=max_length)
    .prop_map(|chars| chars.into_iter().collect::<String>())
    //NOTE: Multibyte characters can exceed the byte limit
    .prop_filter("Generated string exceeds max_bytes", move |s| s.len() <= max_length)
}

#[cfg(test)]
pub fn random_text_length_at_least(min_length: usize) -> impl Strategy<Value = String> {
  proptest::collection::vec(any::<char>(), min_length..=min_length + 10000)
    .prop_map(|chars| chars.into_iter().collect())
}

#[cfg(test)]
pub fn randomize_case(input: &str) -> impl Strategy<Value = String> {
  input.chars().map(|c| {
    if c.is_ascii_alphanumeric() {
      prop_oneof![
        Just(c.to_ascii_uppercase()),
        Just(c.to_ascii_lowercase())
      ]
      .boxed()
    } else {
      Just(c).boxed()
    }
  }).collect::<Vec<_>>().prop_map(|v| v.into_iter().collect())
}

#[cfg(test)]
pub fn pick_values_with_random_case(values: Vec<String>) -> impl Strategy<Value = Vec<String>> {
  let len = values.len();
  random_pick_values(values, 1..=len)
    .prop_flat_map(|values| {
      values.into_iter().map(|v| randomize_case(&v))
        .collect::<Vec<_>>()
        .prop_map(|v| v.into_iter().collect::<Vec<String>>())
    })
}

#[cfg(test)]
macro_rules! pick_values_with_random_case_from {
  ($($values:expr),+ $(,)?) => {{
    let values = vec![$($values),+];
    crate::test_support::string::pick_values_with_random_case(values.iter().map(|s| s.to_string()).collect())
  }};
}

#[cfg(test)]
pub(crate) use pick_values_with_random_case_from;

#[cfg(test)]
pub fn pick_one_with_random_case(values: Vec<String>) -> impl Strategy<Value = String> {
  pick_values_with_random_case(values)
    .prop_map(|v| v.into_iter().next().unwrap())
}

#[cfg(test)]
macro_rules! pick_one_with_random_case_from {
  ($($values:expr),+ $(,)?) => {{
    let values = vec![$($values),+];
    crate::test_support::string::pick_one_with_random_case(values.iter().map(|s| s.to_string()).collect())
  }};
}

#[cfg(test)]
pub(crate) use pick_one_with_random_case_from;