use std::sync::Arc;

use swc_core::common::chain;
use swc_core::common::pass::AndThen;

use crate::bundle::BundleDependencyIndex;
use crate::package::runtime::RuntimeFactory;

use super::ApplyRuntimeCommonJs;
use super::ApplyRuntimeCommonJsReuse;

pub fn apply_runtime_cjs(
  asset_id: String,
  dependency_index: Arc<BundleDependencyIndex>,
  runtime_factory: Arc<RuntimeFactory>,
) -> AndThen<ApplyRuntimeCommonJs, ApplyRuntimeCommonJsReuse> {
  return chain!(
    ApplyRuntimeCommonJs {
      asset_id,
      dependency_index,
      runtime_factory,
    },
    ApplyRuntimeCommonJsReuse {},
  );
}
