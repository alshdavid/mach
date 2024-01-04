use std::collections::HashMap;

use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;

use crate::app_config::AppConfig;
use crate::linking::DependencyIndex;
use crate::public::Asset;
use crate::public::AssetId;
use crate::public::AssetMap;
use crate::public::DependencyMap;
use crate::public::ImportSpecifier;

pub type BundleId = String;
pub type BundleMap = HashMap<BundleId, Bundle>;
pub type BundleDependencyIndex = HashMap<(AssetId, ImportSpecifier), (AssetId, BundleId)>;

// TODO will eventually have Dynamic entries
#[allow(dead_code)]
pub enum BundleKind {
  Entry(AssetId),
  Dynamic,
}

pub struct Bundle {
  pub kind: BundleKind,
  pub assets: Vec<Asset>,
}

pub fn bundle(
  config: &AppConfig,
  asset_map: AssetMap,
  _dependency_map: DependencyMap,
  dependency_index: DependencyIndex,
  source_map: Lrc<SourceMap>,
) -> Result<(Lrc<SourceMap>, BundleMap, BundleDependencyIndex), String> {
  let mut bundles: HashMap<String, Bundle> = BundleMap::new();
  let mut bundle_dependency_index = BundleDependencyIndex::new();

  // Right now, generate a single bundle
  let entry_id = Asset::generate_id(&config.project_root, &config.entry_point);
  let bundle_id = BundleId::from("index");

  let mut bundle = Bundle {
    kind: BundleKind::Entry(entry_id),
    assets: vec![],
  };

  for (_asset_id, asset) in asset_map {
    bundle.assets.push(asset);
  }

  bundles.insert(bundle_id.clone(), bundle);

  for (key, target_asset) in dependency_index {
    bundle_dependency_index.insert(key, (target_asset, bundle_id.clone()));
  }

  return Ok((source_map, bundles, bundle_dependency_index));
}
