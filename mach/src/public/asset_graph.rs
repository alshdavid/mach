use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;

use super::AssetId;
use super::Dependency;
use super::DependencyId;

#[derive(Default)]
pub struct AssetGraph {
  /// This is the dependencies and resolved assets for a given asset
  dependencies: BTreeMap<AssetId, HashSet<(DependencyId, AssetId)>>,
  /// This is the resolved asset for a given dependency
  resolved: HashMap<DependencyId, AssetId>,
}

impl AssetGraph {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_edge(
    &mut self,
    child: AssetId,
    parent: AssetId,
    dependency: DependencyId,
  ) {
    self.resolved.insert(dependency.clone(), parent.clone());
    if let Some(edges) = self.dependencies.get_mut(&child) {
      edges.insert((dependency, parent));
    } else {
      self.dependencies.insert(child, HashSet::from([(dependency, parent)]));
    }
  }

  pub fn get_dependencies(
    &self,
    asset_id: &AssetId,
  ) -> Option<Vec<(&DependencyId, &AssetId)>> {
    let Some(dependencies) = self.dependencies.get(asset_id) else {
      return None;
    };

    let mut result = Vec::<(&DependencyId, &AssetId)>::new();

    for (dependency_id, resolved_asset) in dependencies {
      result.push((dependency_id, resolved_asset));
    }

    return Some(result);
  }

  pub fn get_asset_id_for_dependency(
    &self,
    dependency: &Dependency,
  ) -> Option<AssetId> {
    let Some(asset_id) = self.resolved.get(&dependency.id) else {
      return None;
    };
    return Some(asset_id.clone());
  }
}

impl Debug for AssetGraph {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut map = BTreeMap::<String, Vec<String>>::new();
    for (k, s) in &self.dependencies {
      let mut deps = vec![];
      for (dep_id, asset_id) in s {
        deps.push(format!("Dependency({}) -> Asset({})", dep_id.0.to_string(), asset_id.0.to_string()))
      }
      map.insert(format!("Asset({})", k.0.to_string()), deps);
    }
    f.debug_map().entries(&map).finish()
  }
}
