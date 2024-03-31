use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use libmach::Adapter;
use libmach::AdapterGetPluginOptions;
use libmach::Dependency;
use libmach::Resolver;
use libmach::Transformer;

use tokio::sync::oneshot;

use super::resolve_oxc;
use super::resolver::DenoResolver;
use super::transformer::DenoTransformer;
use super::DenoAction;
use super::DependencyGetters;

pub struct DenoAdapter {
  pub tx: Sender<DenoAction>,
  pub dependency_getters: DependencyGetters,
}

impl Adapter for DenoAdapter {
  fn get_resolver(
    &self,
    options: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Resolver>, String> {
    let resolved = resolve_oxc(&options.specifier, &options.cwd).unwrap();

    let (tx, rx) = oneshot::channel::<()>();
    self.tx.send(DenoAction::LoadResolver(
      resolved.clone(),
      tx,
    )).unwrap();
    rx.blocking_recv().unwrap();

    return Ok(Box::new(DenoResolver {
      tx: self.tx.clone(),
      specifier: resolved.to_str().unwrap().to_string(),
      dependency_getters: self.dependency_getters.clone(),
    }));
  }

  fn get_transformer(
    &self,
    _: AdapterGetPluginOptions,
  ) -> Result<Box<dyn Transformer>, String> {
    return Ok(Box::new(DenoTransformer {}));
  }
}
