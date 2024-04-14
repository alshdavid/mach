use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use super::ipc::NodejsWorker;
use super::ipc::NodejsWorkerFactory;
use super::NodejsInstance;

pub struct NodejsOptions {
  pub workers: usize,
  pub nodejs_worker_factory: Arc<dyn NodejsWorkerFactory>,
}

#[derive(Clone)]
pub struct Nodejs {
  counter: Arc<Mutex<u8>>,
  workers: Vec<NodejsInstance>,
  worker_count: usize,
  nodejs_worker_factory: Arc<dyn NodejsWorkerFactory>,
}

impl Nodejs {
  pub fn new(options: NodejsOptions) -> Self {
    let mut workers = Vec::<NodejsInstance>::new();

    for _ in 0..options.workers {
      let nodejs_instance = NodejsInstance::new(options.nodejs_worker_factory.spawn());
      workers.push(nodejs_instance);
    }
    
    Self {
      counter: Arc::new(Mutex::new(0)),
      workers: workers,
      worker_count: options.workers,
      nodejs_worker_factory: options.nodejs_worker_factory
    }
  }

  pub fn request(
    &self,
    bytes: Vec<u8>,
  ) -> Receiver<Vec<u8>> {
    let next = {
      let mut i = self.counter.lock().unwrap();
      let next = i.clone();
      *i += 1;
      if *i as usize == self.worker_count {
        *i = 0;
      }
      next
    };
    self.workers[next as usize].request(bytes)
  }
}