use std::fmt::Debug;
use std::path::PathBuf;

use crate::Dependency;

pub trait Resolver: Debug + Send + Sync {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String>;
}

#[derive(Clone, Debug)]
pub struct ResolveResult {
  pub file_path: PathBuf,
}
