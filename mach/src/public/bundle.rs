use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::kit::hash::hash_string_sha_256;
use crate::kit::hash::truncate;
use crate::public::ID_TRUNC;

use super::Asset;
use super::AssetId;
use super::BundleId;

#[derive(Default, Clone)]
pub struct Bundle {
  pub id: BundleId,
  pub kind: String,
  pub name: String,
  pub entry_asset: Option<AssetId>,
  pub assets: BTreeMap<PathBuf, (AssetId, String)>,
}

impl Bundle {
  pub fn set_entry_asset(
    &mut self,
    asset: &Asset,
  ) {
    self.entry_asset.replace(asset.id.clone());
  }

  pub fn add_asset(
    &mut self,
    asset: &Asset,
  ) {
    self.assets.insert(
      asset.file_path_relative.clone(),
      (asset.id.clone(), asset.content_hash()),
    );
  }

  pub fn content_hash(&self) -> String {
    let mut content_hashes = String::new();
    for (asset_file_path_relative, (_, asset_content_hash)) in &self.assets {
      let result = format!(
        "{} {}\n",
        asset_file_path_relative.to_str().unwrap(),
        asset_content_hash,
      );
      content_hashes.push_str(&result);
    }

    return truncate(&hash_string_sha_256(&content_hashes), ID_TRUNC);
  }
}

impl std::fmt::Debug for Bundle {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut assets = vec![];
    for (_, (asset_id, _)) in &self.assets {
      assets.push(asset_id.clone())
    }
    f.debug_struct("Bundle")
      .field("id", &self.id.0)
      .field("name", &self.name)
      .field("kind", &self.kind)
      .field("assets", &assets)
      .field("entry_asset", &self.entry_asset)
      .finish()
  }
}
