use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

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
/// This will initialize Nodejs lazily as needed
///
/// The send() method uses a "round-robin" strategy to decide which
/// Nodejs child to send a request to.
#[derive(Clone)]
pub struct NodejsAdapter {
  counter: Arc<Mutex<u8>>,
  worker_count: Arc<u8>,
  nodejs_instance: Arc<Mutex<Option<NodejsInstance>>>,
  tx_to_host: Arc<Sender<(NodejsHostRequest, Sender<NodejsHostResponse>)>>,
  pub rx_host_request:
    Arc<Mutex<Option<Receiver<(NodejsHostRequest, Sender<NodejsHostResponse>)>>>>,
  tx_to_workers: Arc<Vec<Sender<(NodejsClientRequest, Sender<NodejsClientResponse>)>>>,
  rx_to_workers: Arc<Mutex<Vec<Receiver<(NodejsClientRequest, Sender<NodejsClientResponse>)>>>>,
}

impl NodejsAdapter {
  pub fn new(options: NodejsAdapterOptions) -> Result<Self, String> {
    let mut tx_to_workers =
      Vec::<Sender<(NodejsClientRequest, Sender<NodejsClientResponse>)>>::new();
    let mut rx_from_workers =
      Vec::<Receiver<(NodejsClientRequest, Sender<NodejsClientResponse>)>>::new();

    let (tx_worker_host, rx_worker_host) =
      channel::<(NodejsHostRequest, Sender<NodejsHostResponse>)>();

    for _ in 0..options.workers {
      let (tx, rx) = channel();
      tx_to_workers.push(tx);
      rx_from_workers.push(rx);
    }

    Ok(Self {
      counter: Arc::new(Mutex::new(0)),
      worker_count: Arc::new(options.workers),
      nodejs_instance: Arc::new(Mutex::new(None)),
      tx_to_host: Arc::new(tx_worker_host),
      rx_host_request: Arc::new(Mutex::new(Some(rx_worker_host))),
      tx_to_workers: Arc::new(tx_to_workers),
      rx_to_workers: Arc::new(Mutex::new(rx_from_workers)),
    })
  }

  pub fn nodejs_is_running(&self) -> bool {
    self.nodejs_instance.lock().unwrap().is_some()
  }

  pub fn start_nodejs(&self) -> Result<(), String> {
    let mut nodejs_instance_container = self.nodejs_instance.lock().unwrap();
    if nodejs_instance_container.is_some() || *self.worker_count == 0 {
      return Ok(());
    };

    let nodejs_instance = NodejsInstance::new()?;
    let mut rx_to_workers = self.rx_to_workers.lock().unwrap();

    for _ in 0..*self.worker_count {
      let worker = nodejs_instance.spawn_worker();

      // Send messages to child
      thread::spawn({
        let rx_to_worker = rx_to_workers.pop().unwrap();
        let worker_sender = worker.child_sender;

        move || {
          while let Ok((msg, reply)) = rx_to_worker.recv() {
            let rx = worker_sender.send(msg).unwrap();
            reply.send(rx.recv().unwrap()).unwrap();
          }
        }
      });

      // Combine messages from child
      thread::spawn({
        let tx_to_host = self.tx_to_host.clone();
        let worker_receiver = worker.child_receiver;

        move || {
          while let Ok(msg) = worker_receiver.recv() {
            tx_to_host.send(msg).unwrap();
          }
        }
      });
    }

    nodejs_instance_container.replace(nodejs_instance);
    return Ok(());
  }

  pub fn send_all(
    &self,
    req: NodejsClientRequest,
  ) {
    let mut requests = vec![];

    for i in 0..*self.worker_count {
      requests.push(self.send_internal(i as usize, req.clone()))
    }

    for request in requests {
      request.recv().unwrap();
    }
  }

  pub fn send(
    &self,
    req: NodejsClientRequest,
  ) -> Receiver<NodejsClientResponse> {
    let next = self.get_next();
    self.send_internal(next, req)
  }

  pub fn send_and_wait(
    &self,
    req: NodejsClientRequest,
  ) -> NodejsClientResponse {
    self.send(req).recv().unwrap()
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

  fn send_internal(
    &self,
    index: usize,
    req: NodejsClientRequest,
  ) -> Receiver<NodejsClientResponse> {
    let (tx, rx) = channel();
    self.tx_to_workers[index].send((req, tx)).unwrap();
    rx
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
