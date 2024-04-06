use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use super::BundleBehavior;
use super::DependencyPriority;
use super::ImportSymbol;
use super::SpecifierType;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DependencyOptions {
  pub specifier: String,
  pub specifier_type: SpecifierType,
  pub priority: DependencyPriority,
  pub resolve_from: PathBuf,
  pub imported_symbols: Vec<ImportSymbol>,
  pub bundle_behavior: BundleBehavior,
}
