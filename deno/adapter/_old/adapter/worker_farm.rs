use std::sync::{mpsc::{channel, Sender}, Arc, Mutex};

use super::{DenoAction, DenoResponse};

pub type DenoWorkerRequest = (DenoAction, Sender<DenoResponse>);

#[derive(Debug)]
pub struct DenoWorkerFarm {
  c: Arc<Mutex<usize>>,
  senders: Arc<Mutex<Vec<Sender<DenoWorkerRequest>>>>,
}

impl DenoWorkerFarm {
  pub fn new(
    senders: Vec::<Sender<DenoWorkerRequest>>,
  ) -> Self {
    Self {
      c: Arc::new(Mutex::new(0)),
      senders: Arc::new(Mutex::new(senders)),
    }
  }

  pub fn send_all(&self, action: DenoAction) {
    let senders = self.senders.lock().unwrap();
    for sender in senders.iter() {
      let (tx, rx) = channel::<DenoResponse>();
      sender.send((action.clone(), tx)).unwrap();
      rx.recv().unwrap();
    }
  }

  pub fn send(&self, action: DenoAction) -> DenoResponse {
    let senders = self.senders.lock().unwrap();
    let mut c = self.c.lock().unwrap();
    let (tx, rx) = channel::<DenoResponse>();
    let sender = senders.get(*c).unwrap();
    sender.send((action, tx)).unwrap();
    *c += 1;
    if *c >= senders.len() {
      *c = 0;
    }
    rx.recv().unwrap()
  }
}
