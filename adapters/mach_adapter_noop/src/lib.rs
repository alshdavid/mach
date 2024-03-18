use async_trait::async_trait;
use libmach::Adapter;
use libmach::AdapterBootstrap;
use libmach::Config;
use libmach::Dependency;
use libmach::MutableAsset;
use libmach::ResolveResult;
use libmach::Resolver;
use libmach::Transformer;
use std::collections::HashMap;

#[no_mangle]
pub extern fn bootstrap() -> Box<dyn Adapter> {
  return Box::new(NoopAdapter{});
}

pub struct NoopAdapter {}

impl Adapter for NoopAdapter {
  fn get_resolver(
    &self,
    _: HashMap<String, String>,
  ) -> Box<dyn Resolver> {
    return Box::new(NoopResolver {});
  }

  fn get_transformer(
    &self,
    _: HashMap<String, String>,
  ) -> Box<dyn Transformer> {
    return Box::new(NoopTransformer {});
  }
}

#[derive(Debug)]
pub struct NoopResolver {}

#[async_trait]
impl Resolver for NoopResolver {
  async fn resolve(
    &self,
    d: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    println!("dep: {}", d.id);
    return Ok(None);
  }
}

#[derive(Debug)]
pub struct NoopTransformer {}

#[async_trait]
impl Transformer for NoopTransformer {
  async fn transform(
    &self,
    _: &mut MutableAsset,
    _: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
