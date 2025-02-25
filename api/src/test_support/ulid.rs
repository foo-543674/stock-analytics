#[cfg(test)]
use super::datetime::datetime;
#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use rand::prelude::*;
#[cfg(test)]
use ulid::Ulid;

#[cfg(test)]
pub fn random_ulid() -> impl Strategy<Value = Ulid> {
  // NOTE: Combine timestamp and random segments.
  (datetime(), proptest::num::u64::ANY).prop_map(|(datetime, seed)| {
    Ulid::from_datetime_with_source(datetime, &mut StdRng::seed_from_u64(seed))
  })
}

#[cfg(test)]
pub fn random_ulid_string() -> impl Strategy<Value = String> {
  random_ulid().prop_map(|ulid| ulid.to_string())
}
