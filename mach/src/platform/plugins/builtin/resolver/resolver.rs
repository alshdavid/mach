use async_trait::async_trait;

use crate::platform::public::Dependency;
use crate::platform::public::ResolveResult;
use crate::platform::public::Resolver;

use super::resolve;

#[derive(Debug)]
pub struct DefaultResolver {}

#[async_trait]
impl Resolver for DefaultResolver {
  async fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    match resolve(&dependency.resolve_from, &dependency.specifier) {
      Ok(file_path) => {
        return Ok(Some(ResolveResult { file_path }));
      }
      Err(err) => {
        return Err(err);
      }
    }
  }
}
