use super::bundle_single::bundle_single;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;

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
