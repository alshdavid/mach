use std::collections::hash_map::Values;
use std::collections::HashMap;

use super::Bundle;
use super::BundleId;

#[derive(Default, Clone)]
pub struct BundleMap {
  bundles: HashMap<BundleId, Bundle>,
}

impl BundleMap {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn insert(
    &mut self,
    bundle: Bundle,
  ) {
    self.bundles.insert(bundle.id.clone(), bundle);
  }

  pub fn iter(&self) -> Values<'_, BundleId, Bundle> {
    self.bundles.values()
  }

  pub fn len(&self) -> usize {
    self.bundles.len()
  }
}

impl std::fmt::Debug for BundleMap {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut values = self.bundles.values().collect::<Vec<&Bundle>>();
    values.sort_by(|a, b| a.id.cmp(&b.id));
    f.debug_list().entries(&values).finish()
  }
}
