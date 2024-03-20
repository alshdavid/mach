use libmach::Adapter;
use libmach::AdapterBootstrapResult;
use libmach::AdapterBootstrapOptions;
use libmach::AdapterOptions;
use libmach::Config;
use libmach::Dependency;
use libmach::MutableAsset;
use libmach::ResolveResult;
use libmach::Resolver;
use libmach::Transformer;
use std::path::Path;
use std::path::PathBuf;

#[no_mangle]
pub extern fn bootstrap(config: AdapterBootstrapOptions) -> AdapterBootstrapResult {
  dbg!(&config);
  let adapter: Box<dyn Adapter> = Box::new(NoopAdapter{});
  return Box::new(Box::new(Ok(adapter)));
}

pub struct NoopAdapter {}

impl Adapter for NoopAdapter {
  fn get_resolver(
    &self,
    config: AdapterOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    // dbg!(&config);
    return Ok(Box::new(NoopResolver{}));
  }

  fn get_transformer(
    &self,
    _: AdapterOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(NoopTransformer{}));
  }

  fn resolve_specifier(
    &self,
    from_path: &Path,
    _: &str,
  ) -> Result<PathBuf, String> {
    return Ok(from_path.to_path_buf());
  }
}

#[derive(Debug)]
pub struct NoopResolver {}

impl Resolver for NoopResolver {
  fn resolve(
    &self,
    d: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    // println!("dep: {}", d.id);
    return Ok(None);
  }
}

#[derive(Debug)]
pub struct NoopTransformer {}

impl Transformer for NoopTransformer {
  fn transform(
    &self,
    _: &mut MutableAsset,
    _: &Config,
  ) -> Result<(), String> {
    return Ok(());
  }
}
