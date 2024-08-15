use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetId(pub InternalId);

impl AssetId {
  pub fn new() -> Self {
    Self(Default::default())
  }
}

impl Debug for AssetId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "AssetId({})", &self.0)
  }
}

impl Display for AssetId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "AssetId({})", &self.0)
  }
}
