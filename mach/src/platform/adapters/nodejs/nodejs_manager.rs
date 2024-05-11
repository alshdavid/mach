use std::sync::Arc;

use ipc_channel_adapter::host::asynch::ChildSender;
use tokio::sync::mpsc::unbounded_channel;
use tokio::sync::oneshot::Receiver as OneshotReceiver;
use tokio::sync::oneshot::Sender as OneshotSender;
use tokio::sync::Mutex;

use super::NodejsInstance;
use crate::public::nodejs::NodejsClientRequest;
use crate::public::nodejs::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

#[derive(Clone)]
pub struct NodejsManagerOptions {
  pub workers: u8,
}

#[derive(Clone)]
pub struct NodejsManager {
  counter: Arc<Mutex<u8>>,
  workers_sender: Arc<Vec<ChildSender<NodejsClientRequest, NodejsClientResponse>>>,
  worker_count: Arc<u8>,
}

impl NodejsManager {
  pub async fn new(options: NodejsManagerOptions) -> Self {
    let (tx, mut rx) =
      unbounded_channel::<(NodejsHostRequest, OneshotSender<NodejsHostResponse>)>();
    let mut workers_sender = vec![];

    let node_instance = NodejsInstance::new();

    for _ in 0..options.workers {
      let mut worker = node_instance.spawn_worker().await;

      tokio::spawn({
        let tx = tx.clone();

        async move {
          while let Some(msg) = worker.child_receiver.recv().await {
            tx.send(msg).unwrap();
          }
        }
      });

      workers_sender.push(worker.child_sender);
    }

    Self {
      counter: Arc::new(Mutex::new(0)),
      workers_sender: Arc::new(workers_sender),
      worker_count: Arc::new(options.workers),
    }
  }

  pub async fn send_all(
    &self,
    req: NodejsClientRequest,
  ) {
    let mut requests = vec![];

    for sender in self.workers_sender.iter() {
      requests.push(sender.send(req.clone()));
    }

    for request in requests {
      request.await.await.unwrap();
    }
  }

  pub async fn send(
    &self,
    req: NodejsClientRequest,
  ) -> OneshotReceiver<NodejsClientResponse> {
    let next = self.get_next().await;
    self.workers_sender[next].send(req).await
  }

  pub async fn send_and_wait(
    &self,
    req: NodejsClientRequest,
  ) -> NodejsClientResponse {
    let next = self.get_next().await;
    self.workers_sender[next].send_and_wait(req).await
  }

  // TODO use an atomicu8
  async fn get_next(&self) -> usize {
    let mut i = self.counter.lock().await;
    let next = i.clone();
    *i += 1;
    if *i == *self.worker_count {
      *i = 0;
    }
    next as usize
  }
}
