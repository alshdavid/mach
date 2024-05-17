use std::collections::HashMap;

use crate::public::AssetMapSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::BuildResult;

pub fn create_build_result(
  asset_map: AssetMapSync,
  bundles: BundleMapSync,
  bundle_manifest: BundleManifestSync,
) -> BuildResult {
  let asset_map = asset_map.read().unwrap();
  let bundles = bundles.read().unwrap();
  let bundle_manifest = bundle_manifest.read().unwrap();

  let mut build_report = BuildResult {
    bundle_manifest: HashMap::new(),
    entries: HashMap::new(),
  };

  for (key, value) in bundle_manifest.iter() {
    build_report
      .bundle_manifest
      .insert(key.clone(), value.clone());
  }

  for (_bundle_id, bundle) in bundles.iter() {
    let Some(asset_id) = &bundle.entry_asset else {
      continue;
    };
    let Some(asset) = asset_map.get(asset_id) else {
      continue;
    };
    let asset_file_path = asset.file_path_relative.to_str().unwrap().to_string();

    build_report
      .entries
      .insert(asset_file_path, bundle.name.clone());
  }

  return build_report;
}
