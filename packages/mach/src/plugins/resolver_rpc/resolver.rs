use std::sync::Arc;

use anyhow;

use crate::plugins::resolver_javascript::resolve;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingRequestResolverLoadConfig;
use crate::public::AdapterOutgoingRequestResolverRegister;
use crate::public::AdapterOutgoingRequestResolverResolve;
use crate::public::AdapterOutgoingResponse;
use crate::public::Dependency;
use crate::public::MachConfig;
use crate::public::ResolveResult;
use crate::public::Resolver;
use crate::public::RpcHost;

#[derive(Debug)]
pub struct ResolverAdapter {
  pub resolver_specifier: String,
  pub adapter: Arc<dyn RpcHost>,
}

impl ResolverAdapter {
  pub fn new(
    config: &MachConfig,
    initial_specifier: &str,
    adapter: Arc<dyn RpcHost>,
  ) -> anyhow::Result<Self> {
    let specifier =
      resolve(&config.project_root, initial_specifier).map_err(|e| anyhow::anyhow!(e))?;
    if !specifier.exists() {
      return Err(anyhow::anyhow!(format!(
        "Plugin not found for specifier: {:?}",
        initial_specifier
      )));
    }
    let specifier = specifier.to_str().unwrap().to_string();

    // adapter.send_all(AdapterOutgoingRequest::ResolverRegister(
    //   AdapterOutgoingRequestResolverRegister {
    //     specifier: specifier.clone(),
    //   },
    // ))?;

    // adapter.send_all(AdapterOutgoingRequest::ResolverLoadConfig(
    //   AdapterOutgoingRequestResolverLoadConfig {
    //     specifier: specifier.clone(),
    //   },
    // ))?;

    // Ok(Self {
    //   resolver_specifier: specifier,
    //   adapter,
    // })
    todo!();
  }
}

impl Resolver for ResolverAdapter {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    todo!();
    // let response = self
    //   .adapter
    //   .send_and_wait(AdapterOutgoingRequest::ResolverResolve(
    //     AdapterOutgoingRequestResolverResolve {
    //       specifier: self.resolver_specifier.clone(),
    //       dependency: dependency.clone(),
    //     },
    //   ))?;

    // if let AdapterOutgoingResponse::Err(error) = response {
    //   return Err(error);
    // }

    // let AdapterOutgoingResponse::ResolverResolve(result) = response else {
    //   return Err("Incorrect response".to_string());
    // };

    // Ok(result.resolve_result)
  }
}
