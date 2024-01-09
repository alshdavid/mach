use super::AssetId;

#[derive(Debug, Clone)]
pub enum DependencyKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  /// Id of the consumer
  pub parent_asset_id: AssetId,
  /// Id of the dependency
  pub target_asset_id: AssetId,
  /// import foo from 'import_specifier'
  ///    This part -> |----------------|
  pub import_specifier: String,
  pub kind: DependencyKind,
}
