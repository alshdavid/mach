use std::collections::HashMap;

use super::Asset;
use super::AssetId;

#[derive(Debug)]
pub struct AssetMap {
  assets: HashMap<AssetId, Asset>,
}

impl AssetMap {
  pub fn new() -> Self {
    return AssetMap {
      assets: HashMap::new(),
    };
  }

  pub fn insert(&mut self, asset: Asset) -> AssetId {
    let asset_id = asset.id();
    self.assets.insert(asset.id(), asset);
    return asset_id;
  }

  pub fn get_mut(&mut self, asset_id: &AssetId) -> Option<&mut Asset> {
    return self.assets.get_mut(asset_id);
  }

  pub fn get(&self, asset_id: &AssetId) -> Option<&Asset> {
    return self.assets.get(asset_id);
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
    };

    let Some(first_key) = first_key else {
      return None;
    };

    return Some(self.assets.remove(&first_key).unwrap());
  }
}
