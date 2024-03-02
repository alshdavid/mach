use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::platform::public;
use crate::platform::public::AssetGraph;
use crate::platform::public::AssetMap;
use crate::platform::public::BundleGraph;
use crate::platform::public::BundleManifest;
use crate::platform::public::Bundles;
use crate::platform::public::DependencyMap;
use crate::platform::public::Packages;

use super::javascript::package_javascript;
use super::javascript::runtime_factory::RuntimeFactory;

pub fn package(
  config: &public::Config,
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &Bundles,
  bundle_graph: &BundleGraph,
  packages: &mut Packages,
) -> Result<(), String> {
  let source_map = Arc::new(SourceMap::default());
  let mut bundle_manifest = BundleManifest::new();

  for bundle in bundles.iter() {
    bundle_manifest.insert(bundle.id.clone(), format!("/{}", bundle.output));
  }

  for bundle in bundles.iter() {
    if bundle.kind == "js" {
      let runtime_factory = RuntimeFactory::new(source_map.clone());
      package_javascript(
        &config,
        &asset_map,
        &dependency_map,
        &asset_graph,
        &bundles,
        &bundle_graph,
        packages,
        &runtime_factory,
        &bundle,
        &bundle_manifest,
      );
    }
    if bundle.kind == "css" {
    }
    if bundle.kind == "html" {
    }
    if bundle.kind == "file" {
    }
  }

  return Ok(());
}
