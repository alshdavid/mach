use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::path::PathBuf;

pub type AssetEdge = (PathBuf, (String, PathBuf));

pub struct AssetGraph {
  edges: HashMap<PathBuf, HashSet<(String, PathBuf)>>,
}

impl AssetGraph {
  pub fn new() -> Self {
    return AssetGraph {
      edges: HashMap::new(),
    };
  }

  pub fn add_edge(
    &mut self,
    from: PathBuf,
    to: (String, PathBuf),
  ) {
    if let Some(edges) = self.edges.get_mut(&from) {
      edges.insert(to);
    } else {
      self.edges.insert(from, HashSet::from([to]));
    }
  }
}

impl Debug for AssetGraph {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_map().entries(&self.edges).finish()
  }
}
