#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use base32;

#[cfg(test)]
pub fn unambiguous_ulid() -> impl Strategy<Value = String> {
  use core::time;
  // NOTE: Timestamp segment. 48bits (6 bytes) string encoded in base32.
  let timestamp_strategy = proptest::num::u64::ANY.prop_map(|ts| {
    let timestamp = ts % (time::Duration::from_secs(2u64.pow(48)).as_nanos() as u64);
    base32::encode(
      base32::Alphabet::Crockford,
      &timestamp.to_be_bytes()[2..],
    ).to_uppercase()
  });

  // NOTE: Random segment. 80bits (10 bytes) string encoded in base32.
  let random_strategy = proptest::collection::vec(proptest::num::u8::ANY, 10).prop_map(|bytes| {
    base32::encode(base32::Alphabet::Crockford, &bytes).to_uppercase()
  });

  // NOTE: Combine timestamp and random segments.
  (timestamp_strategy, random_strategy).prop_map(|(timestamp, random)| {
    format!("{}{}", timestamp, random)
  })
}