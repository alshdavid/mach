use std::path::PathBuf;

use crate::public::{DependencyPriority, SpecifierType};

pub trait Transformer {
  fn transform(&self);
}

pub struct MutableAsset<'a> {
  code: &'a mut String,
  dependencies: &'a mut Vec<DependencyOptions>,
}

impl<'a> MutableAsset<'a> {
  pub fn set_code(&mut self, code: &str) {
    *self.code = code.to_string();
  }

  pub fn add_dependency(&mut self, options: DependencyOptions) {
    self.dependencies.push(options);
  }
}

pub struct DependencyOptions {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub resolve_from: PathBuf,
  pub imported_symbols: Vec<String>,
}
