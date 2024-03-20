use async_trait::async_trait;
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
  return Box::new(Box::pin(async move {
    tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .build()
      .unwrap()
      .block_on(bootstrap_async(config))
  }));
}

async fn bootstrap_async(config: AdapterBootstrapOptions) -> Result<Box<dyn Adapter>, String> {
    dbg!(&config);
    let adapter: Box<dyn Adapter> = Box::new(NoopAdapter{});
    return Ok(adapter);
}


pub struct NoopAdapter {}

#[async_trait]
impl Adapter for NoopAdapter {
  async fn get_resolver(
    &self,
    config: AdapterOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    dbg!(&config);
    tokio::task::spawn(async {
      println!("hi");
    }).await.unwrap();
    return Ok(Box::new(NoopResolver{}));
  }

  async fn get_transformer(
    &self,
    _: AdapterOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(NoopTransformer{}));
  }

  async fn resolve_specifier(
    &self,
    from_path: &Path,
    _: &str,
  ) -> Result<PathBuf, String> {
    return Ok(from_path.to_path_buf());
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
