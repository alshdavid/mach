use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyId(pub InternalId);

impl DependencyId {
  pub fn new() -> Self {
    Self(Default::default())
  }
}

impl Debug for DependencyId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "DependencyId({})", &self.0)
  }
}

impl Display for DependencyId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "Dependency({})", &self.0)
  }
}
