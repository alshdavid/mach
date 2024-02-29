use crate::platform::hash::hash_string_sha_256;
use crate::platform::hash::truncate;

use super::Dependency;
use super::ID_TRUNC;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Default)]
pub struct DependencyMap {
  pub dependencies: HashMap<String, Dependency>,
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
  ) -> String {
    // TODO this can be done faster
    let key = format!(
      "{}:{:?}:{}:{:?}:{:?}",
      dependency.resolve_from.to_str().unwrap(),
      dependency.specifier_type,
      dependency.specifier,
      dependency.priority,
      dependency.imported_symbols
    );
    let dependency_id = truncate(&hash_string_sha_256(&key), ID_TRUNC);
    self.dependencies.insert(dependency_id.clone(), dependency);
    dependency_id
  }

  pub fn get(
    &self,
    dependency_id: &str,
  ) -> Option<&Dependency> {
    return self.dependencies.get(dependency_id);
  }

  pub fn iter(&self) -> impl Iterator<Item = (&String, &Dependency)> {
    return self.dependencies.iter();
  }
}

impl Debug for DependencyMap {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_map().entries(&self.dependencies).finish()
  }
}

/*

  pub fn lookup_dependency_by_specifier(
    &self,
    parent_id: &AssetId,
    import_specifier: &str,
  ) -> Result<AssetId, String> {
    let Some(dependencies) = self.dependencies.get(parent_id) else {
      return Err(format!(
        "Asset has no dependencies: {} from {}",
        import_specifier, parent_id
      ));
    };

    for dependency in dependencies {
      if dependency.import_specifier == import_specifier {
        return Ok(dependency.target_asset_id.clone());
      }
    }

    return Err(format!(
      "Asset does not contain specifier: importing \"{}\" from \"{}\"",
      import_specifier, parent_id
    ));
  }



  pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &Vec<DependencyLegacy>)> {
    return self.dependencies.iter();
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&AssetId, &mut Vec<DependencyLegacy>)> {
    return self.dependencies.iter_mut();
  }
*/
