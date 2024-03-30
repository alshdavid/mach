use std::fmt;

use serde::Deserialize;
use serde::Serialize;
use snowflake::ProcessUniqueId;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InternalId(ProcessUniqueId);

impl InternalId {
  #[allow(dead_code)]
  pub fn new() -> Self {
    Self::default()
  }

  #[allow(dead_code)]
  pub fn to_string(&self) -> String {
    self.0.to_string()
  }
}

impl Default for InternalId {
  fn default() -> Self {
    Self(ProcessUniqueId::new())
  }
}

impl fmt::Debug for InternalId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> fmt::Result {
    fmt::Debug::fmt(&self.0.to_string(), f)
  }
}

impl fmt::Display for InternalId {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    fmt::Display::fmt(&self.0.to_string(), f)
  }
}
