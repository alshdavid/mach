use std::thread;

use crate::public::nodejs::{NodejsClientRequest, NodejsHostResponse};

use super::{NodejsManager, NodejsManagerOptions};

pub struct NodejsAdapterOptions {
  pub workers: u8
}
pub struct NodejsAdapter {
  nodejs_manager: NodejsManager
}

impl NodejsAdapter {
  pub fn new(options: NodejsAdapterOptions) -> Self {
    let nodejs_manager = NodejsManager::new(NodejsManagerOptions{
      workers: options.workers,
    });

    let rx = nodejs_manager.on.subscribe();
    thread::spawn(move || {
      while let Ok((req, res)) = rx.recv() {
        println!("From child {:?}", req);
        res.send(NodejsHostResponse::Ping).unwrap();
      }
    });

    Self {
      nodejs_manager
    }
  }

  pub fn ping(&self) {
    self.nodejs_manager.send_all(NodejsClientRequest::Ping);
  }

  pub fn resolver_register(&self, specifier: &str) {
    self.nodejs_manager.send_all(NodejsClientRequest::ResolverRegister(specifier.to_string()));
  }
}