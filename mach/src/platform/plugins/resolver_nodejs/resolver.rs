use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::platform::plugins::resolver_javascript::resolve_str;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientRequestResolverRegister;
use crate::public::nodejs::client::NodejsClientRequestResolverResolve;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::Dependency;
use crate::public::MachConfig;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverNodejs {
  pub resolver_specifier: String,
  pub nodejs_adapter: NodejsAdapter,
}

impl ResolverNodejs {
  pub fn new(
    config: &MachConfig,
    specifier: &str,
    nodejs_adapter: NodejsAdapter,
  ) -> Result<Self, String> {
    let specifier = resolve_str(&config.project_root, specifier)?;

    nodejs_adapter.send_all(NodejsClientRequest::ResolverRegister(
      NodejsClientRequestResolverRegister {
        specifier: specifier.clone(),
      },
    ));

    Ok(Self {
      resolver_specifier: specifier.to_string(),
      nodejs_adapter,
    })
  }
}

impl Resolver for ResolverNodejs {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let response = self
      .nodejs_adapter
      .send_and_wait(NodejsClientRequest::ResolverResolve(
        NodejsClientRequestResolverResolve {
          specifier: self.resolver_specifier.clone(),
          dependency: dependency.clone(),
        },
      ));

    let NodejsClientResponse::ResolverResolve(result) = response else {
      panic!();
    };

    Ok(result.resolve_result)
  }
}
