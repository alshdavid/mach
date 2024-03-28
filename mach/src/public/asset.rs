use std::fmt::Debug;
use std::path::PathBuf;

use crate::kit::hash::hash_sha_256;

use super::AssetId;
use super::BundleBehavior;

#[derive(Clone, Default)]
pub struct Asset {
  /// Internal identifier
  pub id: AssetId,
  /// The file name without the extension
  pub name: String,
  /// Absolute filepath to the asset
  pub file_path_absolute: PathBuf,
  /// Relative filepath to the asset
  pub file_path_relative: PathBuf,
  /// Describes the type of the Asset. Starts as the file extension
  pub kind: String,
  /// The body of the Asset in bytes
  pub content: Vec<u8>,
  /// How the asset behaves within the bundle
  pub bundle_behavior: BundleBehavior,
}

impl Asset {
  pub fn content_hash(&self) -> String {
    // Todo, cache this
    return hash_sha_256(&self.content);
  }
}

impl Debug for Asset {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Asset")
      .field("id", &self.id.0)
      .field("file_path", &self.file_path_absolute)
      .field("file_path_rel", &self.file_path_relative)
      .field("bundle_behavior", &self.bundle_behavior)
      .field("kind", &self.kind)
      .finish()
  }
}
