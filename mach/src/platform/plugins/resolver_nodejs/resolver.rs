use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientRequestResolverRun;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::Dependency;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverNodejs {
  pub resolver_specifier: String,
  pub nodejs_adapter: NodejsAdapter,
}

impl Resolver for ResolverNodejs {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let response = self
      .nodejs_adapter
      .send_and_wait(NodejsClientRequest::ResolverRun(NodejsClientRequestResolverRun {
        specifier: self.resolver_specifier.clone(),
        dependency: dependency.clone()
      }));

    let NodejsClientResponse::ResolverRun(result) = response else {
      panic!();
    };

    Ok(result.resolve_result)
  }
}
