/*
  NodejsIpcAdapter holds the Nodejs child process and the IPC channels
  for each worker.

  This will initialize Nodejs lazily as needed

  The send() method uses a "round-robin" strategy to decide which
  Nodejs child to send a request to.
*/

use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use super::NodejsInstance;
use crate::public::Adapter;
use crate::public::AdapterIncomingRequest;
use crate::public::AdapterIncomingResponse;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

#[derive(Clone)]
pub struct NodejsIpcAdapter {
  counter: Arc<Mutex<u8>>,
  worker_count: Arc<u8>,
  nodejs_instance: Arc<Mutex<Option<NodejsInstance>>>,
  tx_to_host: Arc<Sender<(AdapterIncomingRequest, Sender<AdapterIncomingResponse>)>>,
  pub rx_host_request:
    Arc<Mutex<Option<Receiver<(AdapterIncomingRequest, Sender<AdapterIncomingResponse>)>>>>,
  tx_to_workers: Arc<Vec<Sender<(AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>)>>>,
  rx_to_workers:
    Arc<Mutex<Vec<Receiver<(AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>)>>>>,
}

impl NodejsIpcAdapter {
  pub fn new(workers: u8) -> Self {
    let mut tx_to_workers =
      Vec::<Sender<(AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>)>>::new();
    let mut rx_from_workers =
      Vec::<Receiver<(AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>)>>::new();

    let (tx_worker_host, rx_worker_host) =
      channel::<(AdapterIncomingRequest, Sender<AdapterIncomingResponse>)>();

    for _ in 0..workers {
      let (tx, rx) = channel();
      tx_to_workers.push(tx);
      rx_from_workers.push(rx);
    }

    Self {
      counter: Arc::new(Mutex::new(0)),
      worker_count: Arc::new(workers),
      nodejs_instance: Arc::new(Mutex::new(None)),
      tx_to_host: Arc::new(tx_worker_host),
      rx_host_request: Arc::new(Mutex::new(Some(rx_worker_host))),
      tx_to_workers: Arc::new(tx_to_workers),
      rx_to_workers: Arc::new(Mutex::new(rx_from_workers)),
    }
  }
}

impl Adapter for NodejsIpcAdapter {
  fn is_running(&self) -> bool {
    self.nodejs_instance.lock().unwrap().is_some()
  }

  fn init(&self) -> Result<(), String> {
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

  fn send_all(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String> {
    let mut requests = vec![];
    let mut responses = vec![];

    for i in 0..*self.worker_count {
      requests.push(self.send_internal(i as usize, req.clone()))
    }

    for request in requests {
      let response = request.recv().unwrap();
      if let AdapterOutgoingResponse::Err(msg) = response {
        return Err(msg);
      }
      responses.push(response);
    }

    Ok(responses)
  }

  fn send(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse> {
    let next = self.get_next();
    self.send_internal(next, req)
  }

  fn send_and_wait(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<AdapterOutgoingResponse, String> {
    if let Ok(resp) = self.send(req).recv() {
      return Ok(resp);
    }
    Err("Recv Error".to_string())
  }
}

impl NodejsIpcAdapter {
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
    req: AdapterOutgoingRequest,
  ) -> Receiver<AdapterOutgoingResponse> {
    let (tx, rx) = channel();
    self.tx_to_workers[index].send((req, tx)).unwrap();
    rx
  }
}

impl Debug for NodejsIpcAdapter {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("NodejsIpcAdapter")
      .field("worker_count", &self.worker_count)
      .finish()
  }
}
