use std::cell::RefCell;
use std::sync::Arc;

use swc_core::common::SourceMap;

use super::Asset;
use super::Config;
use super::DependencyKind;

pub struct TransformerContext<'a> {
  pub config: &'a Config,
  pub source_map: Arc<SourceMap>,
  dependencies: RefCell<&'a mut Vec<(String, DependencyKind)>>
}

impl<'a> TransformerContext<'a> {
  pub fn new(
    config: &'a Config,
    source_map: Arc<SourceMap>,
    dependencies: &'a mut Vec<(String, DependencyKind)>,
  ) -> Self {
    return TransformerContext{
      config,
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
        f.debug_struct("TransformerContext").field("config", &self.config).field("source_map", &"SourceMap").field("dependencies", &self.dependencies).finish()
    }
}

pub enum TransformResult {
  Convert(Asset),
  End,
  Next,
  Err(String),
}

pub trait Transformer: Send {
  fn transform(&self, ctx: &TransformerContext, asset: &mut Asset) -> TransformResult;
}
