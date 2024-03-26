use super::Dependency;
use super::DependencyId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;

#[derive(Default)]
pub struct DependencyMap {
  pub dependencies: HashMap<DependencyId, Dependency>,
}

impl DependencyMap {
  pub fn new() -> Self {
    DependencyMap {
      dependencies: HashMap::new(),
    }
  }

  pub fn insert(
    &mut self,
    dependency: Dependency,
  ) {
    self.dependencies.insert(dependency.id.clone(), dependency);
  }

  pub fn get(
    &self,
    dependency_id: &DependencyId,
  ) -> Option<&Dependency> {
    return self.dependencies.get(dependency_id);
  }

  // pub fn get_dependency_for_specifier<'a>(
  //   &'a self,
  //   from_asset_id: &Path,
  //   specifier: &str,
  // ) -> Option<&'a Dependency> {
  //   // TODO this can be done more efficiently
  //   for (_, dependency) in &self.dependencies {
  //     if dependency.specifier == *specifier && dependency.resolve_from_rel == from_asset_id {
  //       return Some(dependency);
  //     }
  //   }
  //   return None;
  // }
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
