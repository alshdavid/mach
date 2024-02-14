use async_trait::async_trait;

use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;

use super::resolve;

#[derive(Debug)]
pub struct DefaultResolver {}

#[async_trait]
impl Resolver for DefaultResolver {
  async fn resolve(&self, dependency: &Dependency) -> Result<Option<ResolveResult>, String> {
    println!("ok");
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
