use std::rc::Rc;

use vfs::FileSystem;

use super::AssetContents;
use super::AssetGraph;

#[derive(Clone, Debug)]
pub struct Compilation {
  pub asset_contents: AssetContents,
  pub asset_graph: AssetGraph,
  // pub file_system: Rc<dyn FileSystem>,
}

impl Default for Compilation {
  fn default() -> Self {
    Self {
      asset_contents: Default::default(),
      asset_graph: Default::default(),
      // file_system: Default::default(),
    }
  }
}

impl Compilation {
  pub fn new() -> Self {
    Self {
      ..Default::default()
    }
  }
}
