use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;
use std::sync::Mutex;

use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Outputs;

use super::css::package_css;
use super::html::package_html;
use super::javascript::package_javascript;
use super::javascript::runtime_factory::RuntimeFactory;

pub async fn package(
  config: &public::Config,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  bundles: &mut Bundles,
  bundle_graph: &mut BundleGraph,
  asset_map: &mut AssetMap,
  outputs: &mut Outputs,
) -> Result<(), String> {
  let config_local = Arc::new(config.clone());
  let dependency_map_local = Arc::new(std::mem::take(dependency_map));
  let asset_graph_local = Arc::new(std::mem::take(asset_graph));
  let bundles_local = Arc::new(std::mem::take(bundles));
  let bundle_graph_local = Arc::new(std::mem::take(bundle_graph));
  let asset_map_local = Arc::new(Mutex::new(std::mem::take(asset_map)));
  let outputs_local = Arc::new(Mutex::new(std::mem::take(outputs)));
  let source_map = Arc::new(SourceMap::default());
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  let bundle_manifest = {
    let mut bundle_manifest = BundleManifest::new();
    for bundle in bundles_local.iter() {
      bundle_manifest.insert(bundle.id.clone(), bundle.name.clone());
    }
    Arc::new(bundle_manifest)
  };

  for bundle in bundles_local.iter() {
    let config_local = config_local.clone();
    let asset_map_local = asset_map_local.clone();
    let dependency_map_local = dependency_map_local.clone();
    let asset_graph_local = asset_graph_local.clone();
    let bundles_local = bundles_local.clone();
    let bundle_graph_local = bundle_graph_local.clone();
    let outputs_local = outputs_local.clone();
    let runtime_factory = runtime_factory.clone();
    let bundle = bundle.clone();
    let bundle_manifest = bundle_manifest.clone();

    if bundle.kind == "js" {
      package_javascript(
        config_local,
        asset_map_local,
        dependency_map_local,
        asset_graph_local,
        bundles_local,
        bundle_graph_local,
        outputs_local,
        runtime_factory,
        bundle,
        bundle_manifest,
      );
    } else if bundle.kind == "css" {
      package_css(
        config_local,
        asset_map_local,
        dependency_map_local,
        asset_graph_local,
        bundles_local,
        bundle_graph_local,
        outputs_local,
        bundle,
        bundle_manifest,
      )
    } else if bundle.kind == "html" {
      package_html(
        config_local,
        asset_map_local,
        dependency_map_local,
        asset_graph_local,
        bundles_local,
        bundle_graph_local,
        outputs_local,
        bundle,
        bundle_manifest,
      );
    }
  }

  let bundle_manifest_json = serde_json::to_string_pretty(&*bundle_manifest).unwrap();

  outputs_local.lock().unwrap().push(public::Output {
    content: bundle_manifest_json.as_bytes().to_vec(),
    filepath: PathBuf::from("bundle_manifest.json"),
  });

  // let bundle_graph_json = serde_json::to_string_pretty(&bundle_graph).unwrap();

  // outputs.push(public::Output {
  //   content: bundle_graph_json.as_bytes().to_vec(),
  //   filepath: PathBuf::from("bundle_graph.json"),
  // });

  *dependency_map = Arc::try_unwrap(dependency_map_local).unwrap();
  *asset_graph = Arc::try_unwrap(asset_graph_local).unwrap();
  *bundles = Arc::try_unwrap(bundles_local).unwrap();
  *bundle_graph = Arc::try_unwrap(bundle_graph_local).unwrap();
  *asset_map = Arc::try_unwrap(asset_map_local)
    .unwrap()
    .into_inner()
    .unwrap();
  *outputs = Arc::try_unwrap(outputs_local)
    .unwrap()
    .into_inner()
    .unwrap();

  return Ok(());
}
