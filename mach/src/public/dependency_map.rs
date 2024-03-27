use super::AssetId;
use super::Dependency;
use super::DependencyId;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default)]
pub struct DependencyMap {
  pub dependencies: HashMap<DependencyId, Dependency>,
  pub specifiers: HashMap<(AssetId, String), DependencyId>,
}

impl DependencyMap {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(
    &mut self,
    dependency: Dependency,
  ) {
    self.specifiers.insert((dependency.source_asset.clone(), dependency.specifier.clone()), dependency.id.clone());
    self.dependencies.insert(dependency.id.clone(), dependency);
  }

  pub fn get(
    &self,
    dependency_id: &DependencyId,
  ) -> Option<&Dependency> {
    return self.dependencies.get(dependency_id);
  }

  pub fn get_dependency_for_specifier<'a>(
    &'a self,
    source_asset_id: &AssetId,
    specifier: &str,
  ) -> Option<&'a Dependency> {
    let Some(dependency_id) = self.specifiers.get(&(source_asset_id.clone(), specifier.to_string())) else {
      return None;
    };
    return self.get(&dependency_id);
  }
}

impl Debug for DependencyMap {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut values = self.dependencies.values().collect::<Vec<&Dependency>>();
    values.sort_by(|a, b| a.id.cmp(&b.id));
    f.debug_list().entries(&values).finish()
  }
}
