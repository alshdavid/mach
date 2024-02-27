use std::fmt::Debug;

use async_trait::async_trait;

#[async_trait]
pub trait Packager: Debug + Send + Sync {
  async fn package(&self) -> Result<(), String>;
}
