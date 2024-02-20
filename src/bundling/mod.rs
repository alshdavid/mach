use std::collections::HashSet;
use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::ENTRY_ASSET;

pub fn bundle(
  _config: &public::Config,
  _asset_map: &mut AssetMap,
  _dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  bundles: &mut Bundles,
) -> Result<(), String> {
  // Create one bundle
  let (_, entry_asset_id) = *asset_graph
    .get_dependencies(&ENTRY_ASSET)
    .unwrap()
    .get(0)
    .unwrap();
  let mut bundle = Bundle {
    assets: HashSet::new(),
    entry_asset: entry_asset_id.clone(),
  };

  let mut q = Vec::<PathBuf>::from([entry_asset_id.clone()]);

  while let Some(asset_id) = q.pop() {
    bundle.assets.insert(asset_id.clone());

    let Some(dependencies) = asset_graph.get_dependencies(&asset_id) else {
      continue;
    };

    for (_, asset_id) in dependencies {
      q.push(asset_id.clone());
    }
  }

  bundles.push(bundle);
  return Ok(());
}
