use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use libmach;
use libmach::AssetGraph;
use libmach::AssetMap;
use libmach::Bundle;
use libmach::BundleGraph;
use libmach::BundleManifest;
use libmach::Bundles;
use libmach::DependencyMap;
use libmach::Output;
use libmach::Outputs;
use libmach::Config as MachConfig;

pub fn package_css(
  _config: Arc<MachConfig>,
  asset_map: Arc<Mutex<AssetMap>>,
  _dependency_map: Arc<DependencyMap>,
  _asset_graph: Arc<AssetGraph>,
  _bundles: Arc<Bundles>,
  _bundle_graph: Arc<BundleGraph>,
  outputs: Arc<Mutex<Outputs>>,
  bundle: Bundle,
  _bundle_manifest: Arc<BundleManifest>,
) {
  let mut bundle_content = String::new();

  for asset_id in &bundle.assets {
    let mut asset_map = asset_map.lock().unwrap();
    let asset = asset_map.get_mut(&asset_id).unwrap();
    let contents = std::mem::take(&mut asset.content);
    bundle_content.push_str(&String::from_utf8(contents).unwrap())
  }

  outputs.lock().unwrap().push(Output {
    content: bundle_content.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
