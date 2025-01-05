use ulid::Ulid;

pub fn default_ulid() -> Ulid {
  Ulid::from_string("00000000000000000000000000").unwrap()
}

pub trait StringExt {
  fn is_ulid(&self) -> bool;
}

impl StringExt for str {
  fn is_ulid(&self) -> bool {
    Ulid::from_string(self).is_ok()
  }
}