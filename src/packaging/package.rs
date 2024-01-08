use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::platform::Container;
use crate::public;
use crate::public::Asset;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleMap;
use crate::public::DependencyMap;

use super::apply_runtime_cjs::apply_runtime_cjs;
use super::apply_runtime_esm::fold::apply_runtime_esm;
use super::RuntimeFactory;
use super::optimize::optimize;

pub fn package(
  config: &public::Config,
  asset_map_ref: &mut Container<AssetMap>,
  dependency_map_ref: &mut Container<DependencyMap>,
  bundle_map_ref: &mut Container<BundleMap>,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let mut asset_map = asset_map_ref.take();
  let dependency_map = dependency_map_ref.take_arc();
  let mut bundle_map = bundle_map_ref.take();
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  let mut updated_assets = Vec::<Asset>::new();
  while let Some(asset) = asset_map.pop() {
    let Asset::JavaScript(mut asset) = asset else {
      continue;
    };

    let dependency_map = dependency_map.clone();
    let runtime_factory = runtime_factory.clone();
    let source_map = source_map.clone();

    let asset = swc_core::common::GLOBALS.set(&Globals::new(), move || {
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

      match &mut asset.program {
        Program::Module(m) => {
          let mut stmts = Vec::<Stmt>::new();
          for mi in &mut m.body {
            let ModuleItem::Stmt(s) = mi else {continue;};
            stmts.push(s.clone());
          }
          let wrapped = runtime_factory.module(&asset.id, true, stmts);
          m.body = vec![ModuleItem::Stmt(wrapped)];
        },
        Program::Script(s) => {
          let mut stmts = Vec::<Stmt>::new();
          for s in &s.body {
            stmts.push(s.clone());
          }
          let wrapped = runtime_factory.module(&asset.id, true, stmts);
          s.body = vec![wrapped];
        },
      }

      if config.optimize {
        let result = optimize(asset.program, &asset.file_path, source_map.clone()).expect("failed to optimize");
        asset.program = result;
      }

      return asset;
    });

    updated_assets.push(Asset::JavaScript(asset));
  }

  for asset in updated_assets {
    asset_map.insert(asset);
  }

  for bundle in bundle_map.iter_mut() {
    let Bundle::JavaScript(bundle) = bundle else {
      continue;
    };

    bundle.output.body.push(runtime_factory.header());

    for asset_id in &bundle.assets {
      let Asset::JavaScript(asset) = asset_map.get(asset_id).unwrap() else {
        continue;
      };
      let mut program = asset.program.clone();

      match &mut program {
        Program::Module(m) => {
          let ModuleItem::Stmt(stmt) = m.body.pop().unwrap() else { panic!() };
          bundle.output.body.push(stmt);
        },
        Program::Script(s) => {
          bundle.output.body.push(s.body.pop().unwrap());
        },
      }
    }

    if let Some(entry_id) = &bundle.entry {
      if config.optimize {
        bundle.output.body.push(runtime_factory.bootstrap(entry_id));
        // let result   = optimize(runtime_factory.bootstrap(entry_id), &PathBuf::new(), source_map.clone()).expect("failed to optimize");
      } else {

        bundle.output.body.push(runtime_factory.bootstrap(entry_id));
      }
    }
  }

  asset_map_ref.insert(asset_map);
  dependency_map_ref.insert_arc(dependency_map);
  bundle_map_ref.insert(bundle_map);
  Ok(())
}
