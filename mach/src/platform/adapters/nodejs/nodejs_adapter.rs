use super::NodejsManager;
use super::NodejsManagerOptions;
use crate::public::nodejs::NodejsClientRequest;

pub struct NodejsAdapterOptions {
  pub workers: u8,
}
#[derive(Clone)]
pub struct NodejsAdapter {
  nodejs_manager: NodejsManager,
}

impl NodejsAdapter {
  pub async fn new(options: NodejsAdapterOptions) -> Self {
    let (nodejs_manager, _rx_node_manager) = NodejsManager::new(NodejsManagerOptions {
      workers: options.workers,
    })
    .await;

    Self { nodejs_manager }
  }

  pub async fn ping(&self) {
    self
      .nodejs_manager
      .send_all(NodejsClientRequest::Ping { id: 0 })
      .await;
  }

  pub async fn ping_one(&self) {
    self
      .nodejs_manager
      .send_and_wait(NodejsClientRequest::Ping { id: 0 })
      .await;
  }

  pub async fn resolver_register(
    &self,
    specifier: &str,
  ) {
    self
      .nodejs_manager
      .send_all(NodejsClientRequest::ResolverRegister {
        id: 1,
        data: specifier.to_string(),
      })
      .await;
  }
}
