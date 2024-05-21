use std::fmt::Debug;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AssetId(pub InternalId, String);

impl AssetId {
  pub fn new(specifier: &str) -> Self {
    Self(Default::default(), specifier.to_string())
  }
}

impl Debug for AssetId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "AssetId({})", &self.0.to_string())
  }
}

impl Display for AssetId {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    write!(f, "AssetId({})", &self.1.to_string())
  }
}
