use std::collections::BTreeMap;
use std::path::PathBuf;

use petgraph::graph::NodeIndex;

use super::Asset;
use super::AssetId;
use super::Identifier;

pub type BundleId = NodeIndex;

#[derive(Default, Clone)]
pub struct Bundle {
  pub id: Identifier<BundleId>,
  pub kind: String,
  pub entry_asset: Option<AssetId>,
  pub assets: BTreeMap<PathBuf, AssetId>,
}

impl Bundle {
  pub fn insert_asset(
    &mut self,
    asset: &Asset,
  ) -> anyhow::Result<()> {
    self
      .assets
      .insert(asset.file_path.clone(), asset.id.get()?.clone());

    Ok(())
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
      .field("id", &self.id)
      .field("kind", &self.kind)
      .field("assets", &assets)
      .field("entry_asset", &self.entry_asset)
      .finish()
  }
}
