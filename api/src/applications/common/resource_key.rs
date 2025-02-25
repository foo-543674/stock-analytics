use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct ResourceKey {
  pub key: String,
  pub params: Vec<String>,
}

impl ResourceKey {
  pub fn new(key: String, params: Vec<String>) -> Self {
    Self {
      key,
      params,
    }
  }
}

impl std::fmt::Display for ResourceKey {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}({})", self.key, self.params.join(","))
  }
}

macro_rules! resource_key {
  ($key:expr $(, $params:expr)* $(,)?) => {
    $crate::applications::common::resource_key::ResourceKey::new($key.to_string(), vec![$($params.to_string()),*])
  };
}

pub(crate) use resource_key;
