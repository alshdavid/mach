use crate::platform::adapters::nodejs::NodejsAdapter;
use crate::plugins::resolver_javascript::resolve;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientRequestResolverLoadConfig;
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
    initial_specifier: &str,
    nodejs_adapter: NodejsAdapter,
  ) -> Result<Self, String> {
    let specifier = resolve(&config.project_root, initial_specifier)?;
    if !specifier.exists() {
      return Err(format!(
        "Plugin not found for specifier: {:?}",
        initial_specifier
      ));
    }
    let specifier = specifier.to_str().unwrap().to_string();

    nodejs_adapter.send_all(NodejsClientRequest::ResolverRegister(
      NodejsClientRequestResolverRegister {
        specifier: specifier.clone(),
      },
    ))?;

    nodejs_adapter.send_all(NodejsClientRequest::ResolverLoadConfig(
      NodejsClientRequestResolverLoadConfig {
        specifier: specifier.clone(),
      },
    ))?;

    Ok(Self {
      resolver_specifier: specifier,
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

    if let NodejsClientResponse::Err(error) = response {
      return Err(error);
    }

    let NodejsClientResponse::ResolverResolve(result) = response else {
      return Err("Incorrect response".to_string());
    };

    Ok(result.resolve_result)
  }
}
