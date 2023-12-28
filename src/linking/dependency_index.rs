use std::collections::HashMap;

use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::DependencyMap;
use crate::public::ImportSpecifier;

// TODO generate this during linking and embed this into the DependencyMap struct
pub type DependencyIndex = HashMap<(AssetId, ImportSpecifier), AssetId>;

pub fn generate_dependency_index(
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
) -> DependencyIndex {
  let mut dependency_index = DependencyIndex::new();

  for (asset_id, asset) in asset_map {
    let Some(dependencies) = dependency_map.get(asset_id) else {
      continue;
    };
    for (specifier, dependency) in dependencies {
      let target_asset = asset_map.get(&dependency.asset_id).unwrap();
      dependency_index.insert(
        (asset.id.clone(), specifier.clone()),
        target_asset.id.clone(),
      );
    }
  }

  return dependency_index;
}
