use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::SourceMap;

use super::css::package_css;
use super::html::package_html;
use super::javascript::package_javascript;
use super::javascript::runtime_factory::RuntimeFactory;
use crate::public::AssetGraphSync;
use crate::public::AssetMapSync;
use crate::public::BundleGraphSync;
use crate::public::BundleManifestSync;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;
use crate::public::Output;
use crate::public::OutputsSync;

pub fn package(
  config: MachConfigSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
  bundle_map: BundleMapSync,
  bundle_graph: BundleGraphSync,
  bundle_manifest: BundleManifestSync,
  outputs: OutputsSync,
) -> Result<(), String> {
  let asset_map = asset_map;
  let source_map = Arc::new(SourceMap::default());
  let js_runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  {
    let mut bundle_manifest = bundle_manifest.write().unwrap();

    for bundle in bundle_map.read().unwrap().values() {
      bundle_manifest.insert(bundle.content_hash(), bundle.name.clone());
    }
  };

  for bundle in bundle_map.read().unwrap().values() {
    let bundle = bundle.clone();
    let bundle_manifest = bundle_manifest.clone();

    if bundle.kind == "js" {
      package_javascript(
        config.clone(),
        asset_map.clone(),
        asset_graph.clone(),
        dependency_map.clone(),
        bundle_map.clone(),
        bundle_graph.clone(),
        outputs.clone(),
        js_runtime_factory.clone(),
        bundle,
        bundle_manifest,
      );
    } else if bundle.kind == "css" {
      package_css(asset_map.clone(), outputs.clone(), bundle.clone())
    } else if bundle.kind == "html" {
      package_html(
        asset_map.clone(),
        asset_graph.clone(),
        dependency_map.clone(),
        bundle_map.clone(),
        bundle_graph.clone(),
        outputs.clone(),
        bundle,
        bundle_manifest,
        js_runtime_factory.clone(),
      );
    }
  }

  let bundle_manifest_json = serde_json::to_string_pretty(&*bundle_manifest).unwrap();

  outputs.write().unwrap().push(Output {
    content: bundle_manifest_json.as_bytes().to_vec(),
    filepath: PathBuf::from("bundle_manifest.json"),
  });

  return Ok(());
}
