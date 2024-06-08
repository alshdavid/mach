use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;

use crate::public::RpcHost;
use crate::public::AdapterOutgoingRequest;
use crate::public::AdapterOutgoingResponse;

pub type NapiOutgoingData = (AdapterOutgoingRequest, Sender<AdapterOutgoingResponse>);

#[derive(Clone)]
pub struct NodejsNapiAdapter {
  counter: Arc<Mutex<u8>>,
  worker_count: u8,
  tx_to_worker: Vec<Sender<NapiOutgoingData>>,
  rx_to_worker: Arc<Mutex<Option<Vec<Receiver<NapiOutgoingData>>>>>,
  tx_start_worker: Sender<usize>,
  rx_worker_connected: Arc<Mutex<Option<Receiver<Sender<NapiOutgoingData>>>>>,
  initialized: Arc<RwLock<Option<()>>>,
}

impl NodejsNapiAdapter {
  pub fn new(
    tx_start_worker: Sender<usize>,
    rx_worker_connected: Receiver<Sender<NapiOutgoingData>>,
    worker_count: u8,
  ) -> Self {
    let mut tx_to_worker = vec![];
    let mut rx_to_worker = vec![];
    // let (tx_to_worker, rx_to_worker) = channel();

    for _ in 0..worker_count {
      let (tx, rx) = channel();
      tx_to_worker.push(tx);
      rx_to_worker.push(rx);
    }

    Self {
      counter: Arc::new(Mutex::new(0)),
      tx_start_worker,
      worker_count,
      tx_to_worker,
      rx_to_worker: Arc::new(Mutex::new(Some(rx_to_worker))),
      rx_worker_connected: Arc::new(Mutex::new(Some(rx_worker_connected))),
      initialized: Arc::new(RwLock::new(None)),
    }
  }
}

impl RpcHost for NodejsNapiAdapter {
  fn is_running(&self) -> bool {
    self.initialized.read().unwrap().is_some()
  }

  fn init(&self) -> Result<(), String> {
    let mut is_running = self.initialized.write().unwrap();
    if is_running.is_some() {
      return Ok(());
    }
    is_running.replace(());
    let worker_count = self.worker_count.clone();
    let rx_worker_connected = self.rx_worker_connected.lock().unwrap().take().unwrap();
    let tx_start_worker = self.tx_start_worker.clone();
    let mut rx_to_workers = self.rx_to_worker.lock().unwrap().take().unwrap();

    let handle = thread::spawn({
      let worker_count = worker_count.clone();
      move || {
        let mut tx_workers = vec![];

        for _ in 0..worker_count {
          tx_workers.push(rx_worker_connected.recv().unwrap());
        }

        tx_workers
      }
    });

    for i in 0..worker_count {
      tx_start_worker.send(i as usize).unwrap();
    }

    let mut tx_workers = handle.join().unwrap();

    while let Some(tx_worker) = tx_workers.pop() {
      let rx_to_worker = rx_to_workers.pop().unwrap();

      thread::spawn(move || {
        while let Ok(msg) = rx_to_worker.recv() {
          tx_worker.send(msg).unwrap();
        }
      });
    }

    Ok(())
  }

  fn send_all(
    &self,
    req: AdapterOutgoingRequest,
  ) -> Result<Vec<AdapterOutgoingResponse>, String> {
    let mut requests = vec![];
    let mut responses = vec![];

    for i in 0..self.worker_count {
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

impl NodejsNapiAdapter {
  // TODO use an atomicu8
  fn get_next(&self) -> usize {
    let mut i = self.counter.lock().unwrap();
    let next = i.clone();
    *i += 1;
    if *i == self.worker_count {
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
    self.tx_to_worker[index].send((req, tx)).unwrap();
    rx
  }
}

impl Debug for NodejsNapiAdapter {
  fn fmt(
    &self,
    f: &mut Formatter<'_>,
  ) -> std::fmt::Result {
    f.debug_struct("NodejsNapiAdapter").finish()
  }
}
