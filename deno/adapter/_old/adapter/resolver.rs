use std::sync::Arc;

use libmach::Dependency;
use libmach::ResolveResult;
use libmach::Resolver;

use super::DenoAction;
use super::DenoResponse;
use super::DenoWorkerFarm;

#[derive(Debug)]
pub struct DenoResolver {
  pub specifier: String,
  pub worker_farm: Arc<DenoWorkerFarm>,
}

impl Resolver for DenoResolver {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let DenoResponse::RunResolverResolve(result) = self.worker_farm.send(DenoAction::RunResolverResolve(
      self.specifier.clone(),
      dependency.id.to_string(),
    )) else {
      panic!();
    };

    return Ok(result);
  }
}
