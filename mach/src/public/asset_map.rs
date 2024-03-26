#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use super::Asset;
use super::AssetId;

#[derive(Default)]
pub struct AssetMap {
  assets: HashMap<AssetId, Asset>,
  file_paths: HashMap<PathBuf, AssetId>,
}

impl AssetMap {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(
    &mut self,
    asset: Asset,
  ) -> AssetId {
    let asset_id = asset.id.clone();
    self
      .file_paths
      .insert(asset.file_path_absolute.clone(), asset_id.clone());
    self.assets.insert(asset_id.clone(), asset);
    asset_id
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

  pub fn get_asset_id_for_file_path(
    &self,
    path: &Path,
  ) -> Option<&AssetId> {
    return self.file_paths.get(path);
  }

  pub fn get_many(
    &self,
    asset_ids: &[&AssetId],
  ) -> Result<Vec<&Asset>, String> {
    let mut results = Vec::<&Asset>::new();

    for asset_id in asset_ids {
      let Some(asset) = self.get(&asset_id) else {
        return Err(format!("Could not find Asset: {:?}", asset_id));
      };
      results.push(asset);
    }

    return Ok(results);
  }

  pub fn contains(
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
    let mut values = self.assets.values().collect::<Vec<&Asset>>();
    values.sort_by(|a, b| a.id.cmp(&b.id));
    f.debug_list().entries(&values).finish()
  }
}
