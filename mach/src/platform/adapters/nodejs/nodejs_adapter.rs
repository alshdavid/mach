use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use ipc_channel_adapter::host::sync::ChildSender;

use super::NodejsInstance;
use crate::public::nodejs::client::NodejsClientRequest;
use crate::public::nodejs::client::NodejsClientResponse;
use crate::public::nodejs::NodejsHostRequest;
use crate::public::nodejs::NodejsHostResponse;

#[derive(Clone)]
pub struct NodejsAdapterOptions {
  pub workers: u8,
}

/// NodejsAdapter holds the Nodejs child process and the IPC channels
/// for each worker.
///
/// The send() method uses a "round-robin" strategy to decide which
/// Nodejs child to send a request to.
#[derive(Clone)]
pub struct NodejsAdapter {
  counter: Arc<Mutex<u8>>,
  workers_sender: Arc<Vec<ChildSender<NodejsClientRequest, NodejsClientResponse>>>,
  worker_count: Arc<u8>,
  _nodejs_instance: NodejsInstance,
}

impl NodejsAdapter {
  pub fn new(
    options: NodejsAdapterOptions
  ) -> Result<
    (
      Self,
      Receiver<(NodejsHostRequest, Sender<NodejsHostResponse>)>,
    ),
    String,
  > {
    let (tx, rx) = channel::<(NodejsHostRequest, Sender<NodejsHostResponse>)>();

    let mut workers_sender = vec![];

    let nodejs_instance = NodejsInstance::new()?;

    for _ in 0..options.workers {
      let worker = nodejs_instance.spawn_worker();

      thread::spawn({
        let tx = tx.clone();

        move || {
          while let Ok(msg) = worker.child_receiver.recv() {
            tx.send(msg).unwrap();
          }
        }
      });

      workers_sender.push(worker.child_sender);
    }

    Ok((
      Self {
        counter: Arc::new(Mutex::new(0)),
        workers_sender: Arc::new(workers_sender),
        worker_count: Arc::new(options.workers),
        _nodejs_instance: nodejs_instance,
      },
      rx,
    ))
  }

  pub fn send_all(
    &self,
    req: NodejsClientRequest,
  ) {
    let mut requests = vec![];

    for sender in self.workers_sender.iter() {
      requests.push(sender.send(req.clone()).unwrap());
    }

    for request in requests {
      request.recv().unwrap();
    }
  }

  #[allow(unused)]
  pub fn send(
    &self,
    req: NodejsClientRequest,
  ) -> Receiver<NodejsClientResponse> {
    let next = self.get_next();
    self.workers_sender[next].send(req).unwrap()
  }

  pub fn send_and_wait(
    &self,
    req: NodejsClientRequest,
  ) -> NodejsClientResponse {
    let next = self.get_next();
    self.workers_sender[next].send_blocking(req).unwrap()
  }

  // TODO use an atomicu8
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

impl std::fmt::Debug for NodejsAdapter {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("NodejsAdapter")
      .field("worker_count", &self.worker_count)
      .finish()
  }
}
