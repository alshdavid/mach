use std::collections::hash_map::Iter;
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use super::Bundle;
use super::BundleId;

pub type BundleMapSync = Arc<RwLock<BundleMap>>;

#[derive(Default, Clone)]
pub struct BundleMap {
  bundles: HashMap<BundleId, Bundle>,
}

impl BundleMap {
  pub fn insert(
    &mut self,
    bundle: Bundle,
  ) {
    self.bundles.insert(bundle.id.clone(), bundle);
  }

  pub fn values(&self) -> Values<'_, BundleId, Bundle> {
    self.bundles.values()
  }

  pub fn iter(&self) -> Iter<'_, BundleId, Bundle> {
    self.bundles.iter()
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
