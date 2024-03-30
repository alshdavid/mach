use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_css(
  _config: Arc<public::Config>,
  asset_map: Arc<Mutex<AssetMap>>,
  _dependency_map: Arc<DependencyMap>,
  _asset_graph: Arc<AssetGraph>,
  _bundles: Arc<BundleMap>,
  _bundle_graph: Arc<BundleGraph>,
  outputs: Arc<Mutex<Outputs>>,
  bundle: Bundle,
  _bundle_manifest: &BundleManifest,
) {
  let mut bundle_content = String::new();

  for (_, (asset_id, _)) in &bundle.assets {
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
