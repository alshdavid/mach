use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::BundleMap;
use crate::public::DependencyMap;

use super::bundle_single::bundle_single;
use super::bundle_splitting::bundle_with_splitting;

pub fn bundle(
  config: &public::Config,
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &mut BundleMap,
  bundle_graph: &mut BundleGraph,
) -> Result<(), String> {
  if config.bundle_splitting {
    return bundle_with_splitting(
      config,
      asset_map,
      dependency_map,
      asset_graph,
      bundles,
      bundle_graph,
    );
  } else {
    return bundle_single(config, asset_map, asset_graph, bundles, bundle_graph);
  }
}
