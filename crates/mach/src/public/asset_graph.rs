use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

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

  pub fn get_dependencies(
    &self,
    asset_id: &Path,
  ) -> Option<Vec<(&String, &PathBuf)>> {
    let Some(dependencies) = self.edges.get(asset_id) else {
      return None;
    };

    let mut result = Vec::<(&String, &PathBuf)>::new();

    for (dependency_id, resolved_asset) in dependencies {
      result.push((dependency_id, resolved_asset));
    }

    return Some(result);
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
