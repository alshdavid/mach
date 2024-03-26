use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::InternalId;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct DependencyId(InternalId);

#[derive(Clone, Serialize, Deserialize)]
pub struct Dependency {
  pub id: DependencyId,
  pub content_key: String,
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
  pub resolve_from_rel: PathBuf,
  /// Symbols that are imported from this path
  pub imported_symbols: Vec<ImportSymbolType>,
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
      .field("content_key", &self.content_key)
      .field("specifier", &self.specifier)
      .field("specifier_type", &self.specifier_type)
      .field("is_entry", &self.is_entry)
      .field("priority", &self.priority)
      .field("source_path", &self.source_path)
      .field("resolve_from", &self.resolve_from)
      .field("resolve_from_rel", &self.resolve_from_rel)
      .field("imported_symbols", &self.imported_symbols)
      .field("bundle_behavior", &self.bundle_behavior)
      .finish()
  }
}

impl Default for Dependency {
  fn default() -> Self {
    Self {
      id: DependencyId::default(),
      content_key: String::new(),
      specifier: Default::default(),
      specifier_type: SpecifierType::ESM,
      is_entry: Default::default(),
      priority: DependencyPriority::Sync,
      source_path: Default::default(),
      resolve_from: Default::default(),
      resolve_from_rel: Default::default(),
      imported_symbols: Default::default(),
      bundle_behavior: BundleBehavior::Default,
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
  /// import './foo'
  Unnamed,
  /// import { foo } from './foo'
  /// import { foo: bar } from './foo'
  Named(String),
  /// import foo from './foo'
  Default,
  /// import * as foo from './foo'
  Namespace(String),
  /// export * from './foo'
  Reexport,
  /// import('./foo')
  Dynamic,
  /// require('./foo')
  Commonjs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BundleBehavior {
  Default,
  Inline,
  Isolated,
}

impl Default for BundleBehavior {
  fn default() -> Self {
    BundleBehavior::Default
  }
}
