use std::sync::Arc;

use crate::packaging::RuntimeFactory;
use crate::public::DependencyMap;

use super::apply_runtime_esm::ApplyRuntimeEsm;

pub fn apply_runtime_esm(
  asset_id: String,
  dependency_index: Arc<DependencyMap>,
  runtime_factory: Arc<RuntimeFactory>,
) -> ApplyRuntimeEsm {
  return ApplyRuntimeEsm {
    asset_id,
    dependency_index,
    runtime_factory,
  };
}
