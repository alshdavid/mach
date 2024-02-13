use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::platform::hash::hash_string_sha_256;

use super::AssetId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecifierType {
  ESM,
  Commonjs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Dependency {
  pub fn generate_id(
    source_asset_id: &AssetId,
    specifier: &String,
  ) -> String {
    return hash_string_sha_256(&format!("{}{}", source_asset_id, specifier));
  }
}