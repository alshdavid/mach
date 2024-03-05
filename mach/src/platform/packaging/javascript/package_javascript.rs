use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::kit::swc::module_item_to_stmt;
use crate::kit::swc::parse_program;

use crate::kit::swc::render_module;
use crate::public;
use crate::public::AssetGraph;
use crate::public::AssetMap;
use crate::public::Bundle;
use crate::public::BundleGraph;
use crate::public::BundleManifest;
use crate::public::Bundles;
use crate::public::DependencyMap;
use crate::public::Outputs;
use crate::public::Output;

use super::js_runtime::js_runtime::JavaScriptRuntime;
use super::runtime_factory::RuntimeFactory;

pub fn package_javascript(
  _config: &public::Config,
  asset_map: &AssetMap,
  dependency_map: &DependencyMap,
  asset_graph: &AssetGraph,
  bundles: &Bundles,
  bundle_graph: &BundleGraph,
  outputs: &mut Outputs,
  runtime_factory: &RuntimeFactory,
  bundle: &Bundle,
  bundle_manifest: &BundleManifest,
) {
  let source_map = Arc::new(SourceMap::default());
  let mut bundle_module_stmts = Vec::<Stmt>::new();

  for stmt in runtime_factory.prelude("PROJECT_HASH") {
    bundle_module_stmts.push(stmt);
  }

  let asset_map = &asset_map;
  let mut bundle_assets = bundle.assets.iter().collect::<Vec<&PathBuf>>();
  bundle_assets.sort();

  for asset_id in bundle_assets {
    let source_map = &source_map;
    let asset_graph = &asset_graph;
    let dependency_map = &dependency_map;
    let bundle_graph = &bundle_graph;
    let runtime_factory = &runtime_factory;
    let bundle_id = &bundle.id;

    let stmt = swc_core::common::GLOBALS.set(&Globals::new(), move || {
      let asset = asset_map.get(&asset_id).unwrap();

      let mut module = Module {
        span: Span::default(),
        body: vec![],
        shebang: None,
      };

      let parse_result = parse_program(
        &asset.file_path,
        std::str::from_utf8(&asset.content).unwrap(),
        source_map.clone(),
      )
      .unwrap();
      match parse_result.program {
        Program::Module(m) => module.body = m.body,
        Program::Script(s) => {
          module.body = s.body.into_iter().map(|x| ModuleItem::Stmt(x)).collect()
        }
      }

      let mut javascript_runtime = JavaScriptRuntime {
        current_asset_id: asset_id,
        current_bundle_id: bundle_id,
        dependency_map: &dependency_map,
        bundle_graph: &bundle_graph,
        runtime_factory: &runtime_factory,
        asset_graph: &asset_graph,
        asset_map: &asset_map,
      };

      let module = module.fold_with(&mut javascript_runtime);

      let mut bundle_dependencies = HashSet::<String>::new();
      for (dependency_id, dependency) in dependency_map.iter() {
        if dependency.resolve_from_rel == *asset_id {
          let Some(bundle_id) = bundle_graph.get(dependency_id) else {
            continue;
          };
          if *bundle_id == bundle.name {
            continue;
          }
          bundle_dependencies.insert(bundle_id.clone());
        }
      }

      runtime_factory.module(
        bundle_dependencies.len() != 0,
        asset_id.to_str().unwrap(),
        module_item_to_stmt(module.body),
      )
    });

    bundle_module_stmts.push(stmt);
  }


  if let Some(entry_asset_id) = &bundle.entry_asset {
    if bundles.len() > 1 {
      bundle_module_stmts.push(runtime_factory.manifest(&bundle_manifest).unwrap());
      bundle_module_stmts.push(runtime_factory.import_script());
    }
    bundle_module_stmts.extend(runtime_factory.prelude_mach_require());

    bundle_module_stmts.push(runtime_factory.mach_require(
      entry_asset_id.to_str().unwrap(),
      &[],
      None,
    ));
  }

  let bundle_module = Module {
    span: Span::default(),
    body: vec![ModuleItem::Stmt(
      runtime_factory.wrapper(bundle_module_stmts),
    )],
    shebang: None,
  };

  let rendered = render_module(&bundle_module, source_map);

  outputs.push(Output {
    content: rendered.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
