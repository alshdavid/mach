use std::path::Path;

use super::BundleBehavior;
use super::DependencyOptions;
use super::LinkingSymbol;

#[derive(Debug)]
pub struct MutableAsset<'a> {
  pub file_path: &'a Path,
  pub kind: &'a mut String,
  content: &'a mut Vec<u8>,
  dependencies: &'a mut Vec<DependencyOptions>,
  pub linking_symbols: &'a mut Vec<LinkingSymbol>,
  pub bundle_behavior: &'a mut BundleBehavior,
}

impl<'a> MutableAsset<'a> {
  pub fn new(
    file_path: &'a Path,
    kind: &'a mut String,
    content: &'a mut Vec<u8>,
    dependencies: &'a mut Vec<DependencyOptions>,
    linking_symbols: &'a mut Vec<LinkingSymbol>,
    bundle_behavior: &'a mut BundleBehavior,
  ) -> Self {
    return MutableAsset {
      file_path,
      kind,
      content,
      dependencies,
      linking_symbols,
      bundle_behavior,
    };
  }

  #[allow(dead_code)]
  pub fn get_bytes(&mut self) -> &[u8] {
    return self.content;
  }

  #[allow(dead_code)]
  pub fn set_bytes(
    &mut self,
    bytes: Vec<u8>,
  ) {
    *self.content = bytes;
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
