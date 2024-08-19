use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BundleBehavior {
  #[default]
  Default,
  /// The asset will be placed into a new inline bundle. Inline bundles are not written to a separate file, but embedded into the parent bundle.
  Inline,
  /// The asset will be isolated from its parents in a separate bundle. Shared assets will be duplicated.
  Isolated,
}
