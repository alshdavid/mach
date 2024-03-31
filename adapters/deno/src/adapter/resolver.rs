use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use libmach::Dependency;
use libmach::ResolveResult;
use libmach::Resolver;
use tokio::sync::oneshot;

use super::DenoAction;
use super::DependencyGetters;

#[derive(Debug)]
pub struct DenoResolver {
  pub specifier: String,
  pub tx: Sender<DenoAction>,
  pub dependency_getters: DependencyGetters,
}

impl Resolver for DenoResolver {
  fn resolve(
    &self,
    dependency: &Dependency,
  ) -> Result<Option<ResolveResult>, String> {
    let (tx_done, rx_done) = oneshot::channel::<()>();
    let (tx_getter, rx_getter) = channel::<(String, Sender<String>)>();
    
    let dependency_ref = snowflake::ProcessUniqueId::new().to_string();
    self.dependency_getters.lock().unwrap().insert(dependency_ref.clone(), tx_getter.clone());

    self.tx.send(DenoAction::RunResolverResolve(
      self.specifier.clone(), 
      dependency_ref.clone(),
      tx_done,
    )).unwrap();

    while let Ok((key, reply)) = rx_getter.recv() {
      if key == "id" {
        reply.send(dependency.id.to_string()).unwrap();
      }
      if key == "_close" {
        break
      }
    }

    return Ok(None);
  }
}
