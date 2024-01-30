use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::public;
use crate::public::Asset;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleMap;
use crate::public::DependencyMap;

use super::apply_runtime_cjs::apply_runtime_cjs;
use super::apply_runtime_esm::fold::apply_runtime_esm;
use super::optimize::optimize;
use super::RuntimeFactory;

pub fn package(
  config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  bundle_map: &mut BundleMap,
  source_map: Arc<SourceMap>,
) -> Result<(), String> {
  let dependency_map_arc = Arc::new(std::mem::take(dependency_map));
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  let mut updated_assets = Vec::<Asset>::new();
  while let Some(mut asset) = asset_map.pop() {
    let dependency_map_arc = dependency_map_arc.clone();
    let runtime_factory = runtime_factory.clone();
    let source_map = source_map.clone();

    let asset = swc_core::common::GLOBALS.set(&Globals::new(), move || {
      asset.program = asset.program.fold_with(&mut apply_runtime_esm(
        asset.id.clone(),
        dependency_map_arc.clone(),
        runtime_factory.clone(),
      ));

      asset.program = asset.program.fold_with(&mut apply_runtime_cjs(
        asset.id.clone(),
        dependency_map_arc.clone(),
        runtime_factory.clone(),
      ));

      match &mut asset.program {
        Program::Module(m) => {
          let mut stmts = Vec::<Stmt>::new();
          for mi in &mut m.body {
            let ModuleItem::Stmt(s) = mi else {
              continue;
            };
            stmts.push(s.clone());
          }
          let wrapped = runtime_factory.module(&asset.id, true, stmts);
          m.body = vec![ModuleItem::Stmt(wrapped)];
        }
        Program::Script(s) => {
          let mut stmts = Vec::<Stmt>::new();
          for s in &s.body {
            stmts.push(s.clone());
          }
          let wrapped = runtime_factory.module(&asset.id, true, stmts);
          s.body = vec![wrapped];
        }
      }

      if config.optimize {
        let result = optimize(asset.program, &asset.file_path, source_map.clone())
          .expect("failed to optimize");
        asset.program = result;
      }

      return asset;
    });

    updated_assets.push(asset);
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
      let asset = asset_map.get(asset_id).unwrap();
      let mut program = asset.program.clone();

      match &mut program {
        Program::Module(m) => {
          let ModuleItem::Stmt(stmt) = m.body.pop().unwrap() else {
            panic!()
          };
          bundle.output.body.push(stmt);
        }
        Program::Script(s) => {
          bundle.output.body.push(s.body.pop().unwrap());
        }
      }
    }

    if let Some(entry_id) = &bundle.entry {
      if config.optimize {
        let script = optimize(
          Program::Script(runtime_factory.bootstrap_script(entry_id)),
          &PathBuf::new(),
          source_map.clone(),
        )
        .expect("failed to optimize");
        let Program::Script(mut script) = script else {
          panic!()
        };
        bundle.output.body.push(script.body.pop().unwrap());
      } else {
        bundle.output.body.push(runtime_factory.bootstrap(entry_id));
      }
    }
  }

  std::mem::swap(
    dependency_map,
    &mut Arc::try_unwrap(dependency_map_arc).unwrap(),
  );
  Ok(())
}
