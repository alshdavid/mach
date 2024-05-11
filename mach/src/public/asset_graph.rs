use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::AssetId;
use super::Dependency;
use super::DependencyId;

pub type AssetGraphSync = Arc<RwLock<AssetGraph>>;

#[derive(Default)]
pub struct AssetGraph {
  /// This is the dependencies and resolved assets for a given asset
  dependencies: HashMap<AssetId, HashSet<(DependencyId, AssetId)>>,
  /// This is the resolved asset for a given dependency
  resolved: HashMap<DependencyId, AssetId>,
}

impl AssetGraph {
  pub fn add_edge(
    &mut self,
    source: AssetId,
    resolved: AssetId,
    dependency: DependencyId,
  ) {
    self.resolved.insert(dependency.clone(), resolved.clone());
    if let Some(edges) = self.dependencies.get_mut(&source) {
      edges.insert((dependency, resolved));
    } else {
      self
        .dependencies
        .insert(source, HashSet::from([(dependency, resolved)]));
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
        deps.push(format!(
          "AssetId({}) via DependencyId({})",
          asset_id.0.to_string(),
          dep_id.0.to_string(),
        ))
      }
      map.insert(format!("AssetId({})", k.0.to_string()), deps);
    }
    f.debug_map().entries(&map).finish()
  }
}
