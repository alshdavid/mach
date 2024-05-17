use std::sync::Arc;

use crate::plugins::resolver_javascript::resolve;
use crate::public::Adapter;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingRequestResolverLoadConfig;
use crate::public::AdapterOutgoingRequestResolverRegister;
use crate::public::AdapterOutgoingRequestResolverResolve;
use crate::public::AdapterOutgoingResponse;
use crate::public::Dependency;
use crate::public::MachConfig;
use crate::public::ResolveResult;
use crate::public::Resolver;

#[derive(Debug)]
pub struct ResolverAdapter{
  pub resolver_specifier: String,
  pub adapter: Arc<dyn Adapter>,
}

impl ResolverAdapter {
  pub fn new(
    config: &MachConfig,
    initial_specifier: &str,
    adapter: Arc<dyn Adapter>,
  ) -> Result<Self, String> {
    let specifier = resolve(&config.project_root, initial_specifier)?;
    if !specifier.exists() {
      return Err(format!(
        "Plugin not found for specifier: {:?}",
        initial_specifier
      ));
    }
    let specifier = specifier.to_str().unwrap().to_string();

    adapter.send_all(AdapterOutgoingRequest::ResolverRegister(AdapterOutgoingRequestResolverRegister{
      specifier: specifier.clone(),
    }))?;

    adapter.send_all(AdapterOutgoingRequest::ResolverLoadConfig(AdapterOutgoingRequestResolverLoadConfig{
      specifier: specifier.clone(),
    }))?;

    Ok(Self {
      resolver_specifier: specifier,
      adapter,
    })
  }
}

impl Resolver for ResolverAdapter {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let response = self
      .adapter
      .send_and_wait(AdapterOutgoingRequest::ResolverResolve(AdapterOutgoingRequestResolverResolve{
        specifier: self.resolver_specifier.clone(),
        dependency: dependency.clone(),
      }))?;

    if let AdapterOutgoingResponse::Err(error) = response {
      return Err(error);
    }

    let AdapterOutgoingResponse::ResolverResolve(result) = response else {
      return Err("Incorrect response".to_string());
    };

    Ok(result.resolve_result)
  }
}
