#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;

use super::Asset;
use super::AssetId;

#[derive(Default)]
pub struct AssetMap {
  assets: HashMap<AssetId, Asset>,
  assets_index: HashMap<PathBuf, AssetId>,
}

impl AssetMap {
  pub fn new() -> Self {
    return AssetMap {
      assets: HashMap::new(),
      assets_index: HashMap::new(),
    };
  }

  pub fn insert(
    &mut self,
    asset: Asset,
  ) -> AssetId {
    let asset_id = asset.id.clone();
    let asset_path = asset.file_path.clone();
    self.assets.insert(asset_id.clone(), asset);
    self.assets_index.insert(asset_path, asset_id.clone());
    return asset_id;
  }

  pub fn get_mut(
    &mut self,
    asset_id: &AssetId,
  ) -> Option<&mut Asset> {
    return self.assets.get_mut(asset_id);
  }

  pub fn get(
    &self,
    asset_id: &AssetId,
  ) -> Option<&Asset> {
    return self.assets.get(asset_id);
  }

  pub fn get_file(
    &self,
    file_path: &PathBuf,
  ) -> Option<&Asset> {
    let Some(asset_id) = self.assets_index.get(file_path) else {
      return None;
    };
    return self.assets.get(asset_id);
  }

  pub fn contains_key(
    &self,
    asset_id: &AssetId,
  ) -> bool {
    return self.assets.contains_key(asset_id);
  }

  pub fn len(&self) -> usize {
    return self.assets.len();
  }

  pub fn iter(&self) -> impl Iterator<Item = &Asset> {
    return self.assets.values();
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Asset> {
    return self.assets.values_mut();
  }

  pub fn pop(&mut self) -> Option<Asset> {
    let mut first_key = None;

    for (k, _) in self.assets.iter() {
      first_key.replace(k.clone());
      break;
    }

    let Some(first_key) = first_key else {
      return None;
    };

    return Some(self.assets.remove(&first_key).unwrap());
  }
}

impl Debug for AssetMap {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let values = self.assets.values().collect::<Vec<&Asset>>();
    f.debug_list().entries(&values).finish()
    // f.debug_struct("AssetMap").field("assets", &self.assets).finish()
  }
}
