use std::fmt::Debug;
use std::path::PathBuf;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;

use crate::public::Config;
use crate::public::SpecifierType;

use super::BundleBehavior;
use super::DependencyPriority;
use super::ExportSymbol;
use super::ImportSymbolType;

#[async_trait]
pub trait Transformer: Debug + Send + Sync {
  async fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String>;
}

#[derive(Debug)]
pub struct MutableAsset<'a> {
  pub file_path: PathBuf,
  content: &'a mut Vec<u8>,
  dependencies: &'a mut Vec<DependencyOptions>,
  exports: &'a mut Vec<ExportSymbol>,
}

impl<'a> MutableAsset<'a> {
  pub fn new(
    file_path: PathBuf,
    content: &'a mut Vec<u8>,
    dependencies: &'a mut Vec<DependencyOptions>,
    exports: &'a mut Vec<ExportSymbol>,
  ) -> Self {
    return MutableAsset {
      file_path,
      content,
      dependencies,
      exports,
    };
  }

  pub fn get_code(&mut self) -> String {
    return String::from_utf8(self.content.to_owned()).unwrap();
  }

  pub fn set_code(
    &mut self,
    code: &str,
  ) {
    *self.content = code.as_bytes().to_vec();
  }

  pub fn add_dependency(
    &mut self,
    options: DependencyOptions,
  ) {
    self.dependencies.push(options);
  }

  pub fn define_export(
    &mut self,
    export: ExportSymbol,
  ) {
    self.exports.push(export);
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyOptions {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub resolve_from: PathBuf,
  pub imported_symbols: Vec<ImportSymbolType>,
  pub bundle_behavior: BundleBehavior,
}
