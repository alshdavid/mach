use libmach::Adapter;
use libmach::AdapterGetPluginOptions;
use libmach::Resolver;
use libmach::Transformer;

use crate::resolver::NoopResolver;
use crate::transformer::NoopTransformer;

pub struct NoopAdapter {}

impl Adapter for NoopAdapter {
  fn get_resolver(
    &self,
    config: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    dbg!(&config);
    return Ok(Box::new(NoopResolver{}));
  }

  fn get_transformer(
    &self,
    _: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(NoopTransformer{}));
  }
}
