use std::path::PathBuf;

use dependency::Dependency;

use crate::public::dependency;

pub trait Resolver {
    fn resolve(
      &self,
      dependency: &Dependency
    ) -> Result<Option<ResolveResult>, String>;
}

pub struct ResolveResult {
  pub file_path: PathBuf
}
