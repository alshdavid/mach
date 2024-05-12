use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverNodejs {
  pub nodejs_adapter: NodejsAdapter
}

#[async_trait::async_trait]
impl Resolver for ResolverNodejs {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    // println!("{:?}", self.nodejs_adapter.resolver_run(dependency.clone()).await);
    Ok(None)
  }
}
