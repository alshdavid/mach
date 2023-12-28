use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use swc_core::common::sync::Lrc;
use swc_core::common::SourceMap;
use swc_core::common::Span;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::FoldWith;

use crate::bundle::BundleDependencyIndex;
use crate::bundle::BundleKind;
use crate::bundle::BundleMap;
use crate::config::AppConfig;
use crate::package::RuntimeFactory;

use super::apply_runtime_cjs::apply_runtime_cjs;
use super::apply_runtime_esm::apply_runtime_esm;

pub type OutFile = PathBuf;
pub type PackagedBundles = HashMap<OutFile, Module>;

pub fn package(
  _config: &AppConfig,
  bundle_map: BundleMap,
  dependency_index: BundleDependencyIndex,
  source_map: Lrc<SourceMap>,
) -> Result<(Lrc<SourceMap>, PackagedBundles), String> {
  let runtime_factory = Arc::new(RuntimeFactory::new(source_map.clone()));
  let mut packages = PackagedBundles::new();
  let dependency_index = Arc::new(dependency_index);

  for (bundle_id, mut bundle) in bundle_map {
    let mut package = Module {
      span: Span::default(),
      body: vec![],
      shebang: None,
    };

    for module_item in runtime_factory.prelude() {
      package.body.push(module_item);
    }

    bundle.assets.sort_by(|a, b| a.id.cmp(&b.id));

    for mut asset in bundle.assets {
      // Convert ESM
      asset.ast = asset.ast.fold_with(&mut apply_runtime_esm(
        asset.id.clone(),
        dependency_index.clone(),
        runtime_factory.clone(),
      ));

      // Convert CJS
      asset.ast = asset.ast.fold_with(&mut apply_runtime_cjs(
        asset.id.clone(),
        dependency_index.clone(),
        runtime_factory.clone(),
      ));

      // Wrap the module
      asset.ast.body = vec![ModuleItem::Stmt(runtime_factory.module(
        &asset.id,
        true,
        asset.ast.body,
      ))];

      for module_item in asset.ast.body {
        package.body.push(module_item);
      }
    }

    match bundle.kind {
      BundleKind::Entry(asset_id) => {
        package
          .body
          .push(ModuleItem::Stmt(runtime_factory.import(&asset_id)));
        packages.insert(PathBuf::from(format!("{}.js", bundle_id)), package);
      }
      BundleKind::Dynamic => todo!(),
    }
  }

  return Ok((source_map, packages));
}
