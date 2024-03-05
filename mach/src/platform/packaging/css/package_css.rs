use std::path::PathBuf;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_css(
  _config: &public::Config,
  asset_map: &mut AssetMap,
  _dependency_map: &DependencyMap,
  _asset_graph: &AssetGraph,
  _bundles: &Bundles,
  _bundle_graph: &BundleGraph,
  outputs: &mut Outputs,
  bundle: &Bundle,
  _bundle_manifest: &BundleManifest,
) {
  let mut bundle_content = String::new();

  for asset_id in &bundle.assets {
    let asset = asset_map.get_mut(&asset_id).unwrap();
    let contents = std::mem::take(&mut asset.content);
    bundle_content.push_str(&String::from_utf8(contents).unwrap())
  }

  outputs.push(Output {
    content: bundle_content.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
