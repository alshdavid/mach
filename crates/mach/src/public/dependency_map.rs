use std::collections::HashMap;

use super::AssetId;
use super::Dependency;

#[derive(Debug, Default)]
pub struct DependencyMap {
  pub dependencies: HashMap<AssetId, Vec<Dependency>>,
}

impl DependencyMap {
  pub fn new() -> Self {
    return DependencyMap {
      dependencies: HashMap::new(),
    };
  }

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

  pub fn insert_many(
    &mut self,
    asset_id: &AssetId,
    dependencies: Vec<Dependency>,
  ) {
    let Some(current_dependencies) = self.dependencies.get_mut(asset_id) else {
      self.dependencies.insert(asset_id.clone(), dependencies);
      return;
    };
    current_dependencies.extend(dependencies);
  }

  pub fn insert(
    &mut self,
    asset_id: &AssetId,
    dependency: Dependency,
  ) {
    self.insert_many(asset_id, vec![dependency]);
  }

  pub fn iter(&self) -> impl Iterator<Item = (&AssetId, &Vec<Dependency>)> {
    return self.dependencies.iter();
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = (&AssetId, &mut Vec<Dependency>)> {
    return self.dependencies.iter_mut();
  }
}
