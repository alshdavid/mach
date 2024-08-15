use anyhow;

use crate::plugins::resolver_javascript::resolve;
use crate::types::Dependency;
use crate::types::MachConfig;
use crate::types::ResolveResult;
use crate::types::Resolver;
use crate::rpc::RpcHostRef;

#[derive(Debug)]
pub struct ResolverAdapter {
  pub resolver_specifier: String,
  pub adapter: RpcHostRef,
}

impl ResolverAdapter {
  pub fn new(
    config: &MachConfig,
    initial_specifier: &str,
    _rpc_host: RpcHostRef,
  ) -> anyhow::Result<Self> {
    let specifier =
      resolve(&config.project_root, initial_specifier).map_err(|e| anyhow::anyhow!(e))?;
    if !specifier.exists() {
      return Err(anyhow::anyhow!(format!(
        "Plugin not found for specifier: {:?}",
        initial_specifier
      )));
    }
    let _specifier = specifier.to_str().unwrap().to_string();

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
    _dependency: &Dependency,
  ) -> anyhow::Result<Option<ResolveResult>> {
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
