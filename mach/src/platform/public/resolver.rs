use std::fmt::Debug;
use std::path::PathBuf;

use async_trait::async_trait;
use dependency::Dependency;

use crate::platform::public::dependency;

#[async_trait]
pub trait Resolver: Debug + Send + Sync {
  async fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String>;
}

#[derive(Clone, Debug)]
pub struct ResolveResult {
  pub file_path: PathBuf,
}
