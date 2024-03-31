use libmach::Dependency;
use libmach::ResolveResult;
use libmach::Resolver;

#[derive(Debug)]
pub struct NoopResolver {}

impl Resolver for NoopResolver {
  fn resolve(
    &self,
    _d: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    // println!("dep: {}", d.id);
    return Ok(None);
  }
}
