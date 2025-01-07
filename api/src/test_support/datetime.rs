#[cfg(test)]
use std::time::{SystemTime, Duration, UNIX_EPOCH};
#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
pub fn datetime() -> impl Strategy<Value = SystemTime> {
  proptest::num::u64::ANY.prop_map(|ts| {
    UNIX_EPOCH + Duration::from_millis(ts)
  })
}
