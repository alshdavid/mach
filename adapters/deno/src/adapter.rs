use libmach::Adapter;
use libmach::AdapterGetPluginOptions;
use libmach::Resolver;
use libmach::Transformer;

use crate::resolver::DenoResolver;
use crate::transformer::DenoTransformer;

pub struct DenoAdapter {}

impl Adapter for DenoAdapter {
  fn get_resolver(
    &self,
    _: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    return Ok(Box::new(DenoResolver {}));
  }

  fn get_transformer(
    &self,
    _: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(DenoTransformer {}));
  }
}
