use std::path::PathBuf;

use super::AssetId;

#[derive(Debug, Clone)]
pub enum DependencyKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Debug, Clone)]
pub struct DependencyLegacy {
  /// Id of the consumer
  pub parent_asset_id: AssetId,
  /// Id of the dependency
  pub target_asset_id: AssetId,
  /// import foo from 'import_specifier'
  ///    This part -> |----------------|
  pub import_specifier: String,
  pub kind: DependencyKind,
}

#[derive(Debug, Clone)]
pub enum SpecifierType {
  ESM,
  Commonjs,
}

#[derive(Debug, Clone)]
pub enum DependencyPriority {
  /// Static import
  Sync,
  /// Dynamic import
  Lazy,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  pub id: String,
  /// Identifier of the import 
  pub specifier: String,
  pub specifier_type: SpecifierType,
  /// Whether the dependency is an entry 
  pub is_entry: bool,
  /// The AssetId of the file importing this dependency
  pub source_asset_id: AssetId,
  /// Path to the file that imported this dependency
  pub source_path: PathBuf,
  /// Path to resolve the specifier from
  pub resolve_from: PathBuf,
  /// Symbols that are imported from this path
  pub imported_symbols: Vec<String>,
}