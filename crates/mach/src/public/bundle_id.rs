use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BundleId(pub InternalId);

impl Debug for BundleId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "BundleId({})", &self.0.to_string())
  }
}

impl Display for BundleId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "BundleId({})", &self.0.to_string())
  }
}
