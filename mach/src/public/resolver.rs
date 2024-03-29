use std::fmt::Debug;
use std::path::PathBuf;

use dependency::Dependency;

use crate::public::dependency;

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
