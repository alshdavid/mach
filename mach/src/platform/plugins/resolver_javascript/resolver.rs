use super::resolve;
use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverJavaScript {}

#[async_trait::async_trait]
impl Resolver for ResolverJavaScript {
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
