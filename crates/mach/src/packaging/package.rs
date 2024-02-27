use std::collections::HashMap;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::platform::swc::parse_module;
use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::PackageType;
use crate::public::Packages;

use super::js_runtime::JavaScriptRuntime;
use super::runtime_factory::RuntimeFactory;

pub fn package(
  _config: &public::Config,
  asset_map: &mut AssetMap,
  dependency_map: &mut DependencyMap,
  asset_graph: &mut AssetGraph,
  bundles: &mut Bundles,
  bundle_graph: &mut BundleGraph,
  packages: &mut Packages,
) -> Result<(), String> {
  let source_map = Arc::new(SourceMap::default());
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));

  for bundle in bundles {
    if bundle.kind != "js" {
      continue;
    }

    let source_map = Arc::new(SourceMap::default());
    let mut bundle_module = Module{ 
      span: Span::default(),
      body: vec![],
      shebang: None 
    };

    let asset_map = &asset_map;
    for asset_id in &bundle.assets {
      let source_map = &source_map;
      let dependency_map = &dependency_map;
      let bundle_graph = &bundle_graph;
      let runtime_factory = &runtime_factory;

      let stmt = swc_core::common::GLOBALS.set(&Globals::new(), move || {
        let asset = asset_map.get(&asset_id).unwrap();

        let parse_result = parse_module(&asset.file_path, std::str::from_utf8(&asset.content).unwrap(), source_map.clone()).unwrap();
        
        let mut module = parse_result.module.fold_with(&mut JavaScriptRuntime{ 
          dependency_map: &dependency_map,
          bundle_graph: &bundle_graph,
          runtime_factory: &runtime_factory,
        });

        let mut stmts = Vec::<Stmt>::new();
        for item in module.body {
          match item {
            ModuleItem::ModuleDecl(_) => {},
            ModuleItem::Stmt(stmt) => stmts.push(stmt),
          }
        }
        let stmt = runtime_factory.module(asset_id.to_str().unwrap(), stmts);

        stmt
      });

      
      bundle_module.body.push(ModuleItem::Stmt(stmt));

    }

    packages.insert(bundle.id.clone(), PackageType::JavaScript((bundle_module, source_map)));
  }

  return Ok(());
}
