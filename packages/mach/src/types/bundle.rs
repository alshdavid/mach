use std::collections::BTreeMap;
use std::path::PathBuf;

use super::Asset;
use super::AssetId;
use super::BundleId;

#[derive(Default, Clone)]
pub struct Bundle {
  pub id: BundleId,
  pub kind: String,
  pub entry_asset: Option<AssetId>,
  pub assets: BTreeMap<PathBuf, AssetId>,
}

impl Bundle {
  pub fn insert_asset(
    &mut self,
    asset: &Asset,
  ) -> Option<AssetId> {
    self
      .assets
      .insert(asset.file_path.clone(), asset.id.clone())
  }
}

impl std::fmt::Debug for Bundle {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut assets = vec![];
    for (_, asset_id) in &self.assets {
      assets.push(asset_id.clone())
    }
    f.debug_struct("Bundle")
      .field("id", &self.id.0)
      .field("kind", &self.kind)
      .field("assets", &assets)
      .field("entry_asset", &self.entry_asset)
      .finish()
  }
}
