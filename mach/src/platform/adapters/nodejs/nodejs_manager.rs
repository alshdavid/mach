use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread};

use crate::{kit::broadcast_channel::BroadcastChannel, public::nodejs::{NodejsClientRequest, NodejsClientResponse, NodejsHostRequest, NodejsHostResponse}};

use super::NodejsWorker;


#[derive(Clone)]
pub struct NodejsManagerOptions {
  pub workers: u8
}

// Todo improve performance
pub struct NodejsManager {
  pub on: BroadcastChannel<(NodejsHostRequest, Sender<NodejsHostResponse>)>,
  counter: Arc<Mutex<u8>>,
  workers: Arc<Vec<NodejsWorker>>,
  worker_count: Arc<u8>,
}

impl NodejsManager {
  pub fn new(options: NodejsManagerOptions) -> Self {
    let mut workers = vec![];
    let trx = BroadcastChannel::<(NodejsHostRequest, Sender<NodejsHostResponse>)>::new();
    let (tx, rx) = channel::<(NodejsHostRequest, Sender<NodejsHostResponse>)>();

    for _ in 0..options.workers {
      let worker = NodejsWorker::new();
      trx.merge(worker.child_receiver.on.subscribe());
      workers.push(worker);
    }

    Self {
      on: trx,
      counter: Arc::new(Mutex::new(0)),
      workers: Arc::new(workers),
      worker_count: Arc::new(options.workers),
    }
  }

  pub fn send(&self, req: NodejsClientRequest) -> Receiver<NodejsClientResponse> {
    let next = self.get_next();
    self.workers[next].child_sender.send(req)
  }

  pub fn send_blocking(&self, req: NodejsClientRequest) -> NodejsClientResponse {
    let next = self.get_next();
    self.workers[next].child_sender.send_blocking(req)
  }

  fn get_next(&self) -> usize {
    let mut i = self.counter.lock().unwrap();
    let next = i.clone();
    *i += 1;
    if *i == *self.worker_count {
      *i = 0;
    }
    next as usize
  }
}
