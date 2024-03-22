
use std::fmt::Debug;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use crate::public::SpecifierType;

use super::AssetContent;
use super::BundleBehavior;
use super::DependencyPriority;
use super::ImportSymbolType;

#[derive(Debug)]
pub struct MutableAsset<'a> {
  pub file_path: PathBuf,
  pub kind: &'a mut String,
  content: &'a mut AssetContent,
  dependencies: &'a mut Vec<DependencyOptions>,
}

impl<'a> MutableAsset<'a> {
  pub fn new(
    file_path: PathBuf,
    kind: &'a mut String,
    content: &'a mut AssetContent,
    dependencies: &'a mut Vec<DependencyOptions>,
  ) -> Self {
    return MutableAsset {
      file_path,
      kind,
      content,
      dependencies,
    };
  }

  pub fn get_content(&mut self) -> Result<&mut AssetContent, String> {
    return Ok(self.content);
  }

  pub fn set_content(&mut self, content: AssetContent) {
    *self.content = content;
  }

  pub fn get_bytes(&mut self) -> Result<&mut Vec<u8>, String> {
    let content = self.get_content()?;
    
    if let AssetContent::Bytes(bytes) = content {
      return Ok(bytes);
    }
    return Err(format!("Tried to get content as bytes"));
  }

  pub fn get_str(&mut self) -> Result<&mut str, String> {
    let bytes = self.get_bytes()?;
    let string = std::str::from_utf8_mut(bytes).unwrap();
    return Ok(string);
  }

  #[allow(dead_code)]
  pub fn set_str(
    &mut self,
    code: &String,
  ) {
    self.set_content(AssetContent::Bytes(code.as_bytes().to_vec()));
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
