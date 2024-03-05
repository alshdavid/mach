#![allow(dead_code)]
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use super::Asset;

#[derive(Default)]
pub struct AssetMap {
  assets: HashMap<PathBuf, Asset>,
}

impl AssetMap {
  pub fn new() -> Self {
    return AssetMap {
      assets: HashMap::new(),
    };
  }

  pub fn insert(
    &mut self,
    asset: Asset,
  ) {
    let asset_path = asset.file_path_rel.clone();
    self.assets.insert(asset_path.clone(), asset);
  }

  pub fn get_mut(
    &mut self,
    file_path: &Path,
  ) -> Option<&mut Asset> {
    return self.assets.get_mut(file_path);
  }

  pub fn get(
    &self,
    file_path: &Path,
  ) -> Option<&Asset> {
    return self.assets.get(file_path);
  }

  pub fn get_many(
    &self,
    asset_ids: &[&PathBuf],
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

  pub fn contains_key(
    &self,
    file_path: &Path,
  ) -> bool {
    return self.assets.contains_key(file_path);
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
