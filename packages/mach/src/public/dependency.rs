use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::AssetId;
use super::BundleBehavior;
use super::DependencyId;
use super::DependencyPriority;
use super::ImportSymbol;
use super::ModuleSymbol;
use super::ReexportSymbol;
use super::SpecifierType;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Dependency {
  pub id: DependencyId,
  /// Identifier of the import
  pub specifier: String,
  pub specifier_type: SpecifierType,
  /// Whether the dependency is an entry
  pub is_entry: bool,
  /// When the dependency should be loaded
  pub priority: DependencyPriority,
  /// Path to the file that created this dependency
  pub source_asset_path: PathBuf,
  /// Id of the Asset that created this dependency
  pub source_asset_id: AssetId,
  /// Path to resolve the specifier from
  pub resolve_from: PathBuf,
  /// Symbols that are imported from this target
  pub linking_symbols: Vec<ModuleSymbol>,
  /// Where to place the dependency within the bundle
  pub bundle_behavior: BundleBehavior,
}

impl std::fmt::Debug for Dependency {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("Dependency")
      .field("id", &self.id.0)
      .field("source_asset", &self.source_asset_id.0)
      .field("source_path", &self.source_asset_path)
      .field("resolve_from", &self.resolve_from)
      .field("specifier", &self.specifier)
      .field("specifier_type", &self.specifier_type)
      .field("is_entry", &self.is_entry)
      .field("priority", &self.priority)
      .field("linking_symbols", &self.linking_symbols)
      .field("bundle_behavior", &self.bundle_behavior)
      .finish()
  }
}
