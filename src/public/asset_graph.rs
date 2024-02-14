use std::collections::HashSet;
use std::fmt::Debug;

use super::AssetId;

pub type AssetEdge = (AssetId, AssetId);

pub struct AssetGraph {
  edges: HashSet<AssetEdge>,
}

impl AssetGraph {
  pub fn new() -> Self {
    return AssetGraph {
      edges: HashSet::new(),
    };
  }

  pub fn add_edge(
    &mut self,
    from: AssetId,
    to: AssetId,
  ) {
    self.edges.insert((from, to));
  }
}

impl Debug for AssetGraph {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_list().entries(&self.edges).finish()
  }
}
