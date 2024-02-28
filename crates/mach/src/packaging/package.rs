use std::collections::HashMap;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::platform::swc::parse_module;
use crate::platform::swc::parse_program;
use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::BundleGraph;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::PackageType;
use crate::public::Packages;

use super::js_runtime::js_runtime::JavaScriptRuntime;
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
  let mut manifest = HashMap::<String, String>::new();    

  for bundle in bundles.iter() {
    manifest.insert(bundle.id.clone(), format!("/{}", bundle.output));
  }

  for bundle in bundles.iter() {
    if bundle.kind != "js" {
      continue;
    }

    let source_map = Arc::new(SourceMap::default());
    let mut bundle_module_stmts = Vec::<Stmt>::new();

    for stmt in runtime_factory.prelude("PROJECT_HASH") {
      bundle_module_stmts.push(stmt);
    }
    
    if !bundle.is_lazy {
      bundle_module_stmts.push(runtime_factory.prelude_require_async());
      bundle_module_stmts.push(runtime_factory.import_script());
      bundle_module_stmts.push(runtime_factory.manifest(&manifest).unwrap());
    }

    let asset_map = &asset_map;
    for asset_id in &bundle.assets {
      let source_map = &source_map;
      let asset_graph = &asset_graph;
      let dependency_map = &dependency_map;
      let bundle_graph = &bundle_graph;
      let runtime_factory = &runtime_factory;
      let bundle_id = &bundle.id;

      let stmt = swc_core::common::GLOBALS.set(&Globals::new(), move || {
        let asset = asset_map.get(&asset_id).unwrap();

        let mut module = Module{
            span: Span::default(),
            body: vec![],
            shebang: None,
        };

        let parse_result = parse_program(&asset.file_path, std::str::from_utf8(&asset.content).unwrap(), source_map.clone()).unwrap();
        match parse_result.program {
            Program::Module(m) => module.body = m.body,
            Program::Script(s) => module.body = s.body.into_iter().map(|x| ModuleItem::Stmt(x)).collect(),
        }

        let module = module.fold_with(&mut JavaScriptRuntime{
          current_asset_id: asset_id,
          current_bundle_id: bundle_id,
          dependency_map: &dependency_map,
          bundle_graph: &bundle_graph,
          runtime_factory: &runtime_factory,
          asset_graph: &asset_graph,
        });

        let mut stmts = Vec::<Stmt>::new();
        for item in module.body {
          match item {
            ModuleItem::ModuleDecl(_) => {},
            ModuleItem::Stmt(stmt) => stmts.push(stmt),
          }
        }

        runtime_factory.module(asset_id.to_str().unwrap(), stmts)
      });

      
      bundle_module_stmts.push(stmt);
    }

    if !bundle.is_lazy {
      bundle_module_stmts.push(runtime_factory.require_async(&[], bundle.entry_asset.to_str().unwrap()));
    }

    let bundle_module = Module{ 
      span: Span::default(),
      body: vec![ModuleItem::Stmt(runtime_factory.wrapper(bundle_module_stmts))],
      shebang: None 
    };
    packages.insert(bundle.id.clone(), PackageType::JavaScript((bundle_module, source_map)));
  }

  return Ok(());
}