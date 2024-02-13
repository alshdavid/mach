
use super::Dependency;
use std::fmt::Debug;

#[derive(Default)]
pub struct DependencyGraph {
  pub dependencies: Vec<Dependency>,
}

impl DependencyGraph {
  pub fn new() -> Self {
    return DependencyGraph {
      dependencies: Vec::new(),
    };
  }
  

  pub fn insert(
    &mut self,
    dependency: Dependency,
  ) {
    self.dependencies.push(dependency);
  }
}

impl Debug for DependencyGraph {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_list().entries(&self.dependencies).finish()
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