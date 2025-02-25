#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use std::ops::RangeInclusive;

#[cfg(test)]
pub fn random_pick_values<T: Clone + 'static + std::fmt::Debug>(
  values: Vec<T>,
  count: RangeInclusive<usize>,
) -> impl Strategy<Value = Vec<T>> {
  proptest::collection::vec(proptest::sample::select(values), count)
}

#[cfg(test)]
macro_rules! random_pick_values_from {
  ($($values:expr),+ $(,)?) => {{
    let values = vec![$($values),+];
    crate::test_support::generic::random_pick_values(values.clone(), 1..=values.len())
  }};
}

#[cfg(test)]
pub(crate) use random_pick_values_from;
