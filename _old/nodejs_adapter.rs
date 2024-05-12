use super::NodejsManager;
use super::NodejsManagerOptions;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientRequestPing;
use crate::public::nodejs::client::NodejsClientRequestResolverRegister;
use crate::public::nodejs::client::NodejsClientRequestResolverRun;
use crate::public::nodejs::client::NodejsClientResolverRegister;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::Dependency;
use crate::public::ResolveResult;

pub struct NodejsAdapterOptions {
  pub workers: u8,
}
#[derive(Clone)]
pub struct NodejsAdapter {
  // nodejs_manager: NodejsManager,
}

impl NodejsAdapter {
  pub fn new(options: NodejsAdapterOptions) -> Self {
    // let (nodejs_manager, _rx_node_manager) = NodejsManager::new(NodejsManagerOptions {
    //   workers: options.workers,
    // })
    // .await;

    // Self { nodejs_manager }
    Self{}
  }

  pub fn ping(&self) {
    // self
    //   .nodejs_manager
    //   .send_all(NodejsClientRequest::Ping(NodejsClientRequestPing{}))
    //   .await;
  }

  pub fn ping_one(&self) {
    // self
    //   .nodejs_manager
    //   .send_and_wait(NodejsClientRequest::Ping(NodejsClientRequestPing{}))
    //   .await;
  }

  pub fn resolver_register(
    &self,
    specifier: &str,
  ) {
    // self
    //   .nodejs_manager
    //   .send_all(NodejsClientRequest::ResolverRegister(NodejsClientRequestResolverRegister{
    //     specifier: specifier.to_string()
    //   }))
    //   .await;
  }

  pub fn resolver_run(
    &self,
    dependency: Dependency,
  ) -> Option<ResolveResult> {
    None
    // let response = self
    //   .nodejs_manager
    //   .send_and_wait(NodejsClientRequest::ResolverRun(NodejsClientRequestResolverRun {
    //     dependency
    //   }))
    //   .await;

    // let NodejsClientResponse::ResolverRun(result) = response else {
    //   panic!();
    // };

    // result.resolve_result
  }
}

impl std::fmt::Debug for NodejsAdapter {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.debug_struct("NodejsAdapter").finish()
  }
}
