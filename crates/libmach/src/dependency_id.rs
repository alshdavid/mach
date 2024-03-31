use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyId(pub InternalId);

impl Serialize for DependencyId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl Debug for DependencyId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "DependencyId({})", &self.0.to_string())
  }
}

impl Display for DependencyId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "DependencyId({})", &self.0.to_string())
  }
}
