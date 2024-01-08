use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DependencyKind {
  Static,
  Dynamic,
  Require,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  /// Path to the consumer
  pub consumer_path: PathBuf,
  /// Path to the dependency
  pub target_path: PathBuf,
  /// import foo from 'import_specifier'
  ///    This part -> |----------------|
  pub import_specifier: String,
  pub kind: DependencyKind,
}
