use std::sync::Arc;

use std::sync::RwLock;
use libmach::AssetGraphSync;
use libmach::AssetMapSync;
use libmach::DependencyMapSync;
use libmach::MachConfigSync;
use swc_core::common::SourceMap;

use libmach::BundleGraph;
use libmach::BundleManifest;
use libmach::BundleMap;
use libmach::Outputs;

use super::css::package_css;
use super::html::package_html;
use super::javascript::package_javascript;
use super::javascript::runtime_factory::RuntimeFactory;

pub fn package(
  config: MachConfigSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
  bundles: &mut BundleMap,
  bundle_graph: &mut BundleGraph,
  outputs: &mut Outputs,
) -> Result<(), String> {
  let asset_map = asset_map;
  let bundles_local = Arc::new(std::mem::take(bundles));
  let bundle_graph_local = Arc::new(std::mem::take(bundle_graph));
  let outputs_local = Arc::new(RwLock::new(std::mem::take(outputs)));

  let source_map = Arc::new(SourceMap::default());
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  let bundle_manifest = {
    let mut bundle_manifest = BundleManifest::new();

    for bundle in bundles_local.iter() {
      bundle_manifest.insert(bundle.content_hash(), bundle.name.clone());
    }
    Arc::new(bundle_manifest)
  };

  for bundle in bundles_local.iter() {
    let bundles = bundles_local.clone();
    let bundle_graph = bundle_graph_local.clone();
    let outputs = outputs_local.clone();
    let bundle = bundle.clone();
    let bundle_manifest = bundle_manifest.clone();

    if bundle.kind == "js" {
      package_javascript(
        config.clone(),
        asset_map.clone(),
        asset_graph.clone(),
        dependency_map.clone(),
        bundles,
        bundle_graph,
        outputs,
        runtime_factory.clone(),
        bundle,
        bundle_manifest,
      );
    } else if bundle.kind == "css" {
      package_css(
        asset_map.clone(),
        outputs.clone(),
        bundle.clone(),
      )
    } else if bundle.kind == "html" {
      package_html(
        asset_map.clone(),
        asset_graph.clone(),
        dependency_map.clone(),
        bundles,
        bundle_graph,
        outputs,
        bundle,
        &bundle_manifest,
        runtime_factory.clone(),
      );
    }
  }

  // let bundle_manifest_json = serde_json::to_string_pretty(&*bundle_manifest).unwrap();

  // outputs_local.lock().unwrap().push(public::Output {
  //   content: bundle_manifest_json.as_bytes().to_vec(),
  //   filepath: PathBuf::from("bundle_manifest.json"),
  // });

  // let bundle_graph_json = serde_json::to_string_pretty(&bundle_graph).unwrap();

  // outputs.push(public::Output {
  //   content: bundle_graph_json.as_bytes().to_vec(),
  //   filepath: PathBuf::from("bundle_graph.json"),
  // });

  *bundles = Arc::try_unwrap(bundles_local).unwrap();
  *bundle_graph = Arc::try_unwrap(bundle_graph_local).unwrap();
  *outputs = Arc::try_unwrap(outputs_local)
    .unwrap()
    .into_inner()
    .unwrap();

  return Ok(());
}
