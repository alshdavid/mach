use std::collections::HashMap;

use super::Bundle;

#[derive(Debug)]
pub struct BundleMap {
  bundles: HashMap<String, Bundle>,
}

impl BundleMap {
  pub fn new() -> Self {
    return BundleMap {
      bundles: HashMap::new(),
    };
  }

  pub fn insert(
    &mut self,
    bundle: Bundle,
  ) {
    self.bundles.insert(bundle.name(), bundle);
  }

  pub fn get_mut(
    &mut self,
    name: &String,
  ) -> Option<&mut Bundle> {
    return self.bundles.get_mut(name);
  }

  pub fn get(
    &mut self,
    name: &String,
  ) -> Option<&Bundle> {
    return self.bundles.get(name);
  }

  pub fn iter(&self) -> impl Iterator<Item = &Bundle> {
    return self.bundles.values();
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Bundle> {
    return self.bundles.values_mut();
  }
}
