use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;
use std::sync::Mutex;

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
use crate::public::Output;
use crate::public::Outputs;

use super::js_runtime::js_runtime::JavaScriptRuntime;
use super::runtime_factory::RuntimeFactory;

pub async fn package_javascript(
  _config: Arc<public::Config>,
  asset_map: Arc<Mutex<AssetMap>>,
  dependency_map: Arc<DependencyMap>,
  asset_graph: Arc<AssetGraph>,
  bundles: Arc<Bundles>,
  bundle_graph: Arc<BundleGraph>,
  outputs: Arc<Mutex<Outputs>>,
  runtime_factory: Arc<RuntimeFactory>,
  bundle: Bundle,
  bundle_manifest: Arc<BundleManifest>,
) {
  let source_map = Arc::new(SourceMap::default());
  let mut bundle_module_stmts = Vec::<Stmt>::new();

  for stmt in runtime_factory.prelude("PROJECT_HASH") {
    bundle_module_stmts.push(stmt);
  }

  // let mut bundle_assets = bundle.assets.iter().collect::<Vec<&PathBuf>>();
  // bundle_assets.sort();

  let mut jobs = Vec::new();

  let bundle_id = bundle.id.clone();

  for asset_id in bundle.assets {
    let asset_map = asset_map.clone();
    let source_map = source_map.clone();
    let asset_graph = asset_graph.clone();
    let dependency_map = dependency_map.clone();
    let bundle_graph = bundle_graph.clone();
    let runtime_factory = runtime_factory.clone();
    let bundle_id = bundle_id.clone();
    let bundle_name = bundle.name.clone();
    
    jobs.push(tokio::task::spawn(async move {
      let (asset_file_path, asset_content) = {
        let mut asset_map = asset_map.lock().unwrap();
        let asset = asset_map.get_mut(&asset_id).unwrap();
        (asset.file_path_rel.clone(), std::mem::take(&mut asset.content))
      };

      let mut module = Module {
        span: Span::default(),
        body: vec![],
        shebang: None,
      };

      let parse_result = parse_program(
        &asset_file_path,
        std::str::from_utf8(&asset_content).unwrap(),
        source_map.clone(),
      )
      .unwrap();

      match parse_result.program {
        Program::Module(m) => module.body = m.body,
        Program::Script(s) => module.body = s.body.into_iter().map(|x| ModuleItem::Stmt(x)).collect(),
      }

      let mut javascript_runtime = JavaScriptRuntime {
        current_asset_id: &asset_id,
        current_bundle_id: &bundle_id,
        dependency_map: &dependency_map,
        bundle_graph: &bundle_graph,
        runtime_factory: &runtime_factory,
        asset_graph: &asset_graph,
        asset_map: asset_map.clone(),
        depends_on: HashSet::new(),
      };

      let (module, javascript_runtime) = swc_core::common::GLOBALS.set(&Globals::new(), move || {
        let module = module.fold_with(&mut javascript_runtime);
        return (module, javascript_runtime);
      });

      let stmt = runtime_factory.module(
        javascript_runtime.depends_on.len() != 0,
        asset_id.to_str().unwrap(),
        module_item_to_stmt(module.body),
      );

      (stmt, asset_id)
    }));
  }

  let mut stmts = vec![];

  for job in jobs {
    let result = job.await.unwrap();
    stmts.push(result);
  }

  stmts.sort_by(|a, b| a.1.cmp(&b.1));
  for (stmt, _) in stmts.drain(0..) {
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

  outputs.lock().unwrap().push(Output {
    content: rendered.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}
