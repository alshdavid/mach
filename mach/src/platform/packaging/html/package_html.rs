use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Packages;

pub fn package_html(
  _config: &public::Config,
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &Bundles,
  bundle_graph: &BundleGraph,
  packages: &mut Packages,
  bundle: &Bundle,
  bundle_manifest: &BundleManifest,
) {
  let Some(dependencies) = asset_graph.get_dependencies(&bundle.entry_asset) else {
    return;
  };
  if dependencies.len() == 0 {
    return;
  }
  let Some(asset) = asset_map.get(&bundle.entry_asset) else {
    panic!("could not find asset")
  };
  
  dbg!(&asset);
}
