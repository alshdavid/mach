use libmach::AssetGraphSync;
use libmach::AssetMapSync;
use libmach::BundleGraphSync;
use libmach::BundleMapSync;
use libmach::DependencyMapSync;
use libmach::MachConfigSync;

use super::bundle_single::bundle_single;

pub fn bundle(
  config: MachConfigSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
  bundles: BundleMapSync,
  bundle_graph: BundleGraphSync,
) -> Result<(), String> {
  if config.bundle_splitting {
    todo!();
    // return bundle_with_splitting(
    //   config,
    //   asset_map,
    //   dependency_map,
    //   asset_graph,
    //   bundles,
    //   bundle_graph,
    // );
  } else {
    return bundle_single(
      config,
      asset_map,
      asset_graph,
      dependency_map,
      bundles,
      bundle_graph,
    );
  }
}
