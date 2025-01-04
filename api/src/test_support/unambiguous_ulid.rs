#[cfg(test)]
use proptest::prelude::*;

#[cfg(test)]
pub fn unambiguous_ulid() -> impl Strategy<Value = String> {
    "[0-9a-hjkmnp-zA-HJKMNP-Z]{26}".boxed()
}