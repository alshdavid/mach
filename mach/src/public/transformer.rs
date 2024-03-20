use std::fmt::Debug;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::public::Config;
use crate::public::SpecifierType;

use super::BundleBehavior;
use super::DependencyPriority;
use super::ImportSymbolType;

pub trait Transformer: Debug + Send + Sync {
  fn transform(
    &self,
    asset: &mut MutableAsset,
    config: &Config,
  ) -> Result<(), String>;
}

#[derive(Debug)]
pub struct MutableAsset<'a> {
  pub file_path: PathBuf,
  pub kind: &'a mut String,
  content: &'a mut Vec<u8>,
  dependencies: &'a mut Vec<DependencyOptions>,
}

impl<'a> MutableAsset<'a> {
  pub fn new(
    file_path: PathBuf,
    kind: &'a mut String,
    content: &'a mut Vec<u8>,
    dependencies: &'a mut Vec<DependencyOptions>,
  ) -> Self {
    return MutableAsset {
      file_path,
      kind,
      content,
      dependencies,
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
