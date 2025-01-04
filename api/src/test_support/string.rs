#[cfg(test)]
use proptest::prelude::*;

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