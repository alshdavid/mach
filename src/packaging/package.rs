use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::ModuleItem;
use swc_core::ecma::visit::FoldWith;

use crate::platform::Container;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleMap;
use crate::public::DependencyMap;
use crate::public::JavaScriptAsset;

use super::RuntimeFactory;
use super::apply_runtime_cjs::apply_runtime_cjs;
use super::apply_runtime_esm::fold::apply_runtime_esm;

pub fn package(
  asset_map_ref: &mut Container<AssetMap>,
  dependency_map_ref: &mut Container<DependencyMap>,
  bundle_map_ref: &mut Container<BundleMap>,
  source_map_ref: &mut Container<SourceMap>,
) -> Result<(), String> {
  let asset_map = asset_map_ref.take();
  let dependency_map = dependency_map_ref.take_arc();
  let mut bundle_map = bundle_map_ref.take();
  let source_map = source_map_ref.take_arc();
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  for bundle in bundle_map.iter_mut() {
    let Bundle::JavaScript(bundle) = bundle else { continue };
    
    bundle.output.body.push(runtime_factory.header());
    
    let bundle_assets = Vec::<JavaScriptAsset>::new();

    while let Some(mut asset) = bundle.assets.pop() {
      let runtime_factory = runtime_factory.clone();
      let dependency_map = dependency_map.clone();

      let result = swc_core::common::GLOBALS.set(&Globals::new(), move || {
        asset.program = asset.program.fold_with(&mut apply_runtime_esm(
          asset.id.clone(),
          dependency_map.clone(),
          runtime_factory.clone(),
        ));
  
        asset.program = asset.program.fold_with(&mut apply_runtime_cjs(
          asset.id.clone(),
          dependency_map.clone(),
          runtime_factory.clone(),
        ));
  
        return runtime_factory.module(
          &asset.id,
          true,
          asset.program,
        );
      });
    
      bundle.output.body.push(result);
    }

    bundle.assets = bundle_assets;

    if let Some(entry_id) = &bundle.entry {
      bundle.output.body.push(runtime_factory.bootstrap(entry_id));
    }
  }

  asset_map_ref.insert(asset_map);
  dependency_map_ref.insert_arc(dependency_map);
  bundle_map_ref.insert(bundle_map);
  source_map_ref.insert_arc(source_map);
  Ok(())
}
