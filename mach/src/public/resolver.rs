use std::fmt::Debug;
use std::path::PathBuf;

use dependency::Dependency;
use serde::{Deserialize, Serialize};

use super::dependency;

#[async_trait::async_trait]
pub trait Resolver: Debug + Send + Sync {
  async fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolveResult {
  pub file_path: PathBuf,
}
