use std::cell::RefCell;
use std::sync::Arc;

use swc_core::common::SourceMap;
use swc_core::ecma::ast::Program;

use super::Asset;
use super::AssetMap;
use super::Config;
use super::DependencyKind;
use super::DependencyMap;

pub struct TransformerContext<'a> {
  pub config: &'a Config,
  pub asset_map: &'a AssetMap,
  pub dependency_map: &'a DependencyMap,
  pub source_map: Arc<SourceMap>,
  dependencies: RefCell<&'a mut Vec<(String, DependencyKind)>>
}

impl<'a> TransformerContext<'a> {
  pub fn new(
    config: &'a Config,
    asset_map: &'a AssetMap,
    dependency_map: &'a DependencyMap,
    source_map: Arc<SourceMap>,
    dependencies: &'a mut Vec<(String, DependencyKind)>,
  ) -> Self {
    return TransformerContext{
      config,
      asset_map,
      dependency_map,
      source_map,
      dependencies: RefCell::new(dependencies),
    };
  }

  pub fn add_dependency(&self, specifier: &str, kind: DependencyKind) {
    self.dependencies.borrow_mut().push((specifier.to_string(), kind));
  }
}

impl<'a> std::fmt::Debug for TransformerContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransformerContext").field("config", &self.config).field("asset_map", &self.asset_map).field("dependency_map", &self.dependency_map).field("source_map", &"SourceMap").field("dependencies", &self.dependencies).finish()
    }
}

pub enum TransformResult {
  NewJavaScriptAsset(JavaScriptAssetOptions),
  End,
  Next,
  Err(String),
}

pub trait Transformer: Send {
  fn transform(&self, ctx: &TransformerContext, asset: &mut Asset) -> TransformResult;
}

pub struct JavaScriptAssetOptions {
  pub program: Program,
}
