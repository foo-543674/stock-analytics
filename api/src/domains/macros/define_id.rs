pub use ulid::Ulid;
pub use std::fmt;

pub trait Identitable {
  fn new(value: Ulid) -> Self;
  fn to_string(&self) -> String;
}

macro_rules! define_id {
  ($name:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $name(Ulid);

    impl Identitable for $name {
      fn new(value: Ulid) -> Self {
        $name(value)
      }

      fn to_string(&self) -> String {
        self.0.to_string()
      }
    }

    impl fmt::Display for $name {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
      }
    }
  };
}

pub(crate) use define_id;