use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::path::Path;
use std::path::PathBuf;

use super::Dependency;

#[derive(Default)]
pub struct AssetGraph {
  /// AssetRelPath -> [](DependencyId, AssetRelPath)
  edges: HashMap<PathBuf, HashSet<(String, PathBuf)>>,
  parents: HashMap<String, PathBuf>,
}

impl AssetGraph {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_edge(
    &mut self,
    from: PathBuf,
    to: (String, PathBuf),
  ) {
    self.parents.insert(to.0.clone(), to.1.clone());
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

  pub fn get_asset_id_for_dependency(
    &self,
    dependency: &Dependency,
  ) -> Option<PathBuf> {
    let Some(asset_id) = self.parents.get(&dependency.id) else {
      return None;
    };
    return Some(asset_id.clone());
  }

  pub fn _iter(&self) -> impl Iterator<Item = (&PathBuf, &HashSet<(String, PathBuf)>)> {
    self.edges.iter()
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
