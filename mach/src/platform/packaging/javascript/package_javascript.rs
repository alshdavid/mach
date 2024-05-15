use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread::JoinHandle;

use swc_core::common::Globals;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use super::js_runtime::js_runtime::JavaScriptRuntime;
use super::runtime_factory::RuntimeFactory;
use crate::kit::swc::module_item_to_stmt;
use crate::kit::swc::parse_program;
use crate::kit::swc::render_module;
use crate::public::AssetGraphSync;
use crate::public::AssetId;
use crate::public::AssetMapSync;
use crate::public::Bundle;
use crate::public::BundleGraphSync;
use crate::public::BundleManifest;
use crate::public::BundleMapSync;
use crate::public::DependencyMapSync;
use crate::public::MachConfigSync;
use crate::public::Output;
use crate::public::Outputs;

pub fn package_javascript<'a>(
  config: MachConfigSync,
  asset_map: AssetMapSync,
  asset_graph: AssetGraphSync,
  dependency_map: DependencyMapSync,
  bundle_map: BundleMapSync,
  bundle_graph: BundleGraphSync,
  outputs: Arc<RwLock<Outputs>>,
  runtime_factory: Arc<RuntimeFactory>,
  bundle: Bundle,
  bundle_manifest: Arc<BundleManifest>,
) {
  let bundle_map = bundle_map.read().unwrap();
  let bundle_graph = bundle_graph.read().unwrap();
  let source_map = Arc::new(SourceMap::default());

  let mut assets_to_package = divide_assets_by_threads(&bundle, config.threads);
  let bundle_id = bundle.id.clone();
  let mut handles = Vec::<JoinHandle<Result<(), String>>>::new();
  let stmts = Arc::new(Mutex::new(BTreeMap::<PathBuf, Stmt>::new()));

  for i in 0..config.threads {
    let mut assets = assets_to_package[i].take().unwrap();
    let asset_map = asset_map.clone();
    let source_map = source_map.clone();
    let asset_graph = asset_graph.clone();
    let dependency_map = dependency_map.clone();
    let bundle_graph = bundle_graph.clone();
    let runtime_factory = runtime_factory.clone();
    let bundle_id = bundle_id.clone();
    let stmts = stmts.clone();

    handles.push(std::thread::spawn(move || -> Result<(), String> {
      let asset_graph = asset_graph.read().unwrap();
      let dependency_map = dependency_map.read().unwrap();

      for asset_id in assets.drain(0..) {
        let asset_id = asset_id.clone();

        let (asset_content, _asset_file_path_absolute, asset_file_path_relative) = {
          let mut asset_map = asset_map.write().unwrap();
          let asset = asset_map.get_mut(&asset_id).unwrap();
          (
            std::mem::take(&mut asset.content),
            asset.file_path_absolute.clone(),
            asset.file_path_relative.clone(),
          )
        };

        let mut module = Module {
          span: Span::default(),
          body: vec![],
          shebang: None,
        };

        let parse_result = parse_program(
          &asset_file_path_relative,
          std::str::from_utf8(&asset_content).unwrap(),
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
          current_asset_id: &asset_id,
          current_bundle_id: &bundle_id,
          dependency_map: &dependency_map,
          bundle_graph: &bundle_graph,
          runtime_factory: &runtime_factory,
          asset_graph: &asset_graph,
          asset_map: asset_map.clone(),
          depends_on: HashSet::new(),
        };

        let (module, javascript_runtime) =
          swc_core::common::GLOBALS.set(&Globals::new(), move || {
            let module = module.fold_with(&mut javascript_runtime);
            return (module, javascript_runtime);
          });

        let stmt = runtime_factory.module(
          javascript_runtime.depends_on.len() != 0,
          asset_file_path_relative.to_str().unwrap(),
          module_item_to_stmt(module.body),
        );

        stmts.lock().unwrap().insert(asset_file_path_relative, stmt);
      }

      return Ok(());
    }));
  }

  for handle in handles {
    handle.join().unwrap().unwrap();
  }

  let mut bundle_module_stmts = Vec::<Stmt>::new();

  for stmt in runtime_factory.prelude("PROJECT_HASH", &bundle.name) {
    bundle_module_stmts.push(stmt);
  }

  // Take ownership of stmts and store them
  let stmts = Arc::try_unwrap(stmts).unwrap().into_inner().unwrap();
  bundle_module_stmts.extend(stmts.into_values().collect::<Vec<Stmt>>());

  if let Some(entry_asset_id) = &bundle.entry_asset {
    let entry_asset_filepath_relative = asset_map
      .read()
      .unwrap()
      .get(entry_asset_id)
      .unwrap()
      .file_path_relative
      .clone();
    if bundle_map.len() > 1 {
      bundle_module_stmts.push(runtime_factory.manifest(&bundle_manifest).unwrap());
      bundle_module_stmts.push(runtime_factory.import_script());
    }
    bundle_module_stmts.extend(runtime_factory.prelude_mach_require());

    bundle_module_stmts.push(runtime_factory.mach_require(
      entry_asset_filepath_relative.to_str().unwrap(),
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

  outputs.write().unwrap().push(Output {
    content: rendered.as_bytes().to_vec(),
    filepath: PathBuf::from(&bundle.name),
  });
}

fn divide_assets_by_threads(
  bundle: &Bundle,
  threads: usize,
) -> Vec<Option<Vec<AssetId>>> {
  let mut assets_to_package = Vec::<Option<Vec<AssetId>>>::new();

  for _ in 0..threads {
    assets_to_package.push(Some(Vec::new()));
  }

  let mut t = 0;
  for (_, (asset_id, _)) in bundle.assets.iter() {
    assets_to_package[t]
      .as_mut()
      .unwrap()
      .push(asset_id.clone());

    t += 1;
    if t == threads {
      t = 0;
    }
  }

  return assets_to_package;
}
