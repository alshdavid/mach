use std::sync::Arc;

use libmach::Adapter;
use libmach::AdapterGetPluginOptions;
use libmach::Resolver;
use libmach::Transformer;

use super::resolve_oxc;
use super::resolver::DenoResolver;
use super::transformer::DenoTransformer;
use super::DenoAction;
use super::DenoWorkerFarm;

pub struct DenoAdapter {
  pub worker_farm: Arc<DenoWorkerFarm>,
}

impl Adapter for DenoAdapter {
  fn get_resolver(
    &self,
    options: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    let resolved = resolve_oxc(&options.specifier, &options.cwd).unwrap();

    self.worker_farm.send_all(DenoAction::LoadResolver(resolved.clone()));

    return Ok(Box::new(DenoResolver {
      worker_farm: self.worker_farm.clone(),
      specifier: resolved.to_str().unwrap().to_string(),
    }));
  }

  fn get_transformer(
    &self,
    _: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(DenoTransformer {}));
  }
}
