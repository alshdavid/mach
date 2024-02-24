use std::fmt::Debug;
use std::path::PathBuf;

use super::BundleBehavior;

#[derive(Clone)]
pub struct Asset {
  pub file_path: PathBuf,
  /// Describes the type of the Asset. Stars as the file extension
  pub kind: String,
  pub content: Vec<u8>,
  pub bundle_behavior: BundleBehavior,
  pub exports: Vec<ExportSymbol>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ExportSymbol {
  // export const foo = ''
  // export const { foo, bar } = foobar
  //               |---||---|
  // export { foo }
  // export { foo as bar }
  //                |---|
  Named(String),
  // export default foo
  Default,
  // export * from './foo'
  ExportAll(String),
}

impl Debug for Asset {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Asset")
      .field("file_path", &self.file_path)
      .field("bundle_behavior", &self.bundle_behavior)
      .field("kind", &self.kind)
      .field("exports", &self.exports)
      .finish()
  }
}