use std::collections::HashMap;

use super::AssetId;

pub type ImportSpecifier = String;
pub type DependencyMap = HashMap<AssetId, HashMap<ImportSpecifier, Dependency>>;

#[derive(Debug, Clone)]
pub enum DependencyKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  pub specifier: ImportSpecifier,
  pub asset_id: AssetId,
  pub kind: DependencyKind,
}
