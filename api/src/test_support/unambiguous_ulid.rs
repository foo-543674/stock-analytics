#[cfg(test)]
use proptest::prelude::*;
#[cfg(test)]
use base32;

#[cfg(test)]
pub fn unambiguous_ulid() -> impl Strategy<Value = String> {
  // タイムスタンプ部分（48ビット: 6バイト）

use core::time;
  let timestamp_strategy = proptest::num::u64::ANY.prop_map(|ts| {
    let timestamp = ts % (time::Duration::from_secs(2u64.pow(48)).as_nanos() as u64);
    base32::encode(
      base32::Alphabet::Crockford,
      &timestamp.to_be_bytes()[2..],
    ).to_uppercase()
  });

  // ランダム部分（80ビット: 10バイト）
  let random_strategy = proptest::collection::vec(proptest::num::u8::ANY, 10).prop_map(|bytes| {
    base32::encode(base32::Alphabet::Crockford, &bytes).to_uppercase()
  });

  // タイムスタンプ部分とランダム部分を結合してULIDを生成
  (timestamp_strategy, random_strategy).prop_map(|(timestamp, random)| {
    format!("{}{}", timestamp, random)
  })
}