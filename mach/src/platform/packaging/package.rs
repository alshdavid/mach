use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Packages;

use super::html::package_html;
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
    if bundle.kind == "css" {}
    if bundle.kind == "html" {
      package_html(
        config,
        asset_map,
        dependency_map,
        asset_graph,
        bundles,
        bundle_graph,
        packages,
        bundle,
        &bundle_manifest,
      )
    }
    if bundle.kind == "file" {}
  }

  return Ok(());
}
