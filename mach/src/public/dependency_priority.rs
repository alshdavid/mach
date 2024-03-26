use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum DependencyPriority {
  /// Static import
  #[default]
  Sync,
  /// Dynamic import
  Lazy,
}
