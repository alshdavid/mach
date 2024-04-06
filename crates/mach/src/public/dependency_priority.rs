use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum DependencyPriority {
  /// Static import
  #[default]
  Sync,
  /// Dynamic import
  Lazy,
}
