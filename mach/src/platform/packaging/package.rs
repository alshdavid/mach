use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Outputs;

use super::html::package_html;
use super::css::package_css;
use super::javascript::package_javascript;
use super::javascript::runtime_factory::RuntimeFactory;

pub fn package(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &Bundles,
  bundle_graph: &BundleGraph,
  outputs: &mut Outputs,
) -> Result<(), String> {
  let source_map = Arc::new(SourceMap::default());
  let mut bundle_manifest = BundleManifest::new();

  for bundle in bundles.iter() {
    bundle_manifest.insert(bundle.id.clone(), bundle.name.clone());
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
        outputs,
        &runtime_factory,
        &bundle,
        &bundle_manifest,
      );
    }
  }

  for bundle in bundles.iter() {
    if bundle.kind == "css" {
      package_css(
        &config,
        asset_map,
        &dependency_map,
        &asset_graph,
        &bundles,
        &bundle_graph,
        outputs,
        &bundle,
        &mut bundle_manifest,
      )
    }
  }

  for bundle in bundles.iter() {
    if bundle.kind == "html" {
      package_html(
        config,
        asset_map,
        dependency_map,
        asset_graph,
        bundles,
        bundle_graph,
        outputs,
        bundle,
        &mut bundle_manifest,
      )
    }
  }

  let bundle_manifest_json = serde_json::to_string_pretty(&bundle_manifest).unwrap();

  outputs.push(public::Output { 
    content: bundle_manifest_json.as_bytes().to_vec(),
    filepath: PathBuf::from("bundle_manifest.json"),
  });

  // let bundle_graph_json = serde_json::to_string_pretty(&bundle_graph).unwrap();

  // outputs.push(public::Output { 
  //   content: bundle_graph_json.as_bytes().to_vec(),
  //   filepath: PathBuf::from("bundle_graph.json"),
  // });

  return Ok(());
}
