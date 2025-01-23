pub use ulid::Ulid;
pub use std::fmt;

pub trait Identitable {
  fn to_string(&self) -> String;
}

macro_rules! define_id {
  ($name:ident) => {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct $name(Ulid);

    impl $name {
      pub fn new(id: Ulid) -> Self {
        $name(id)
      }
      
      pub fn from_string(value: &str) -> Result<Self, crate::domains::errors::domain_error::DomainError> {
        ulid::Ulid::from_string(value)
          .map($name)
          .map_err(|e| crate::domains::errors::domain_error::DomainError::InvalidData(e.to_string()))
      }

      pub fn value(&self) -> &Ulid {
        &self.0
      }
    }

    impl Identitable for $name {
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
