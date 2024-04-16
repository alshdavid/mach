use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::public::nodejs::NodejsClientResponse;

use super::NodejsWorker;

pub struct NodejsManagerOptions {
  pub workers: u8
}

// Todo improve performance
#[derive(Clone)]
pub struct NodejsManager {
  counter: Arc<Mutex<u8>>,
  workers: Vec<NodejsWorker>,
  worker_count: u8,
}

impl NodejsManager {
  pub fn new(options: NodejsManagerOptions) -> Self {
    let mut workers = vec![];

    for _ in 0..options.workers {
      workers.push(NodejsWorker::new());
    }

    Self {
      counter: Arc::new(Mutex::new(0)),
      workers: workers,
      worker_count: options.workers,
    }
  }

  pub fn send_ping(&self) -> Receiver<NodejsClientResponse> {
    let next = self.get_next();
    self.workers[next as usize].send_ping()
  }

  fn get_next(&self) -> u8 {
    let mut i = self.counter.lock().unwrap();
    let next = i.clone();
    *i += 1;
    if *i == self.worker_count {
      *i = 0;
    }
    next
  }
}
