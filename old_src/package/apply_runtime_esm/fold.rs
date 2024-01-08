use std::sync::Arc;

use crate::bundle::BundleDependencyIndex;
use crate::package::runtime::RuntimeFactory;

use super::ApplyRuntimeEsm;

pub fn apply_runtime_esm(
  asset_id: String,
  dependency_index: Arc<BundleDependencyIndex>,
  runtime_factory: Arc<RuntimeFactory>,
) -> ApplyRuntimeEsm {
  return ApplyRuntimeEsm {
    asset_id,
    dependency_index,
    runtime_factory,
  };
}
