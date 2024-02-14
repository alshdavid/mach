use std::fmt::Debug;
use std::path::PathBuf;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::public::{Config, SpecifierType};

#[async_trait]
pub trait Transformer: Debug + Send + Sync {
  async fn transform(&self, asset: &mut MutableAsset, config: &Config) -> Result<(), String>;
}

#[derive(Debug)]
pub struct MutableAsset<'a> {
  pub file_path: PathBuf,
  code: &'a mut String,
  dependencies: &'a mut Vec<DependencyOptions>,
}

impl<'a> MutableAsset<'a> {
  pub fn new(
    file_path: PathBuf,
    code: &'a mut String,
    dependencies: &'a mut Vec<DependencyOptions>,
  ) -> Self {
    return MutableAsset {
        file_path,
        code,
        dependencies,
    }
  }

  pub fn get_code(&self) -> &String {
    return self.code;
  }

  pub fn set_code(&mut self, code: &str) {
    *self.code = code.to_string();
  }

  pub fn add_dependency(&mut self, options: DependencyOptions) {
    self.dependencies.push(options);
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyOptions {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub resolve_from: PathBuf,
  pub imported_symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyPriority {
  /// Static import
  Sync,
  /// Dynamic import
  Lazy,
}