use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
  /// Identifier of the import
  pub specifier: String,
  pub specifier_type: SpecifierType,
  /// Whether the dependency is an entry
  pub is_entry: bool,
  /// When the dependency should be loaded
  pub priority: DependencyPriority,
  /// Path to the file that imported this dependency
  pub source_path: PathBuf,
  /// Path to resolve the specifier from
  pub resolve_from: PathBuf,
  /// Symbols that are imported from this path
  pub imported_symbols: Vec<ImportSymbolType>,
  /// Where to place the dependency within the bundle
  pub bundle_behavior: BundleBehavior,
}

impl Default for Dependency {
  fn default() -> Self {
    Self {
      specifier: Default::default(),
      specifier_type: SpecifierType::ESM,
      is_entry: Default::default(),
      priority: DependencyPriority::Sync,
      source_path: Default::default(),
      resolve_from: Default::default(),
      imported_symbols: Default::default(),
      bundle_behavior: BundleBehavior::Inline,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecifierType {
  ESM,
  Commonjs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyPriority {
  /// Static import
  Sync,
  /// Dynamic import
  Lazy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ImportSymbolType {
  // import './foo'
  Unnamed,
  // import { foo } from './foo'
  // import { foo: bar } from './foo'
  Named(String),
  // import foo from './foo'
  Default,
  // import * as foo from './foo'
  Namespace(String),
  // export * from './foo'
  Reexport,
  // import('./foo')
  Dynamic,
  // require('./foo')
  Commonjs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BundleBehavior {
  Inline,
  Isolated,
}
