use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use super::DenoMachRequest;

pub struct DenoWorkerFarm {
  senders: Vec<Sender<DenoMachRequest>>,
  c: usize,
}

impl DenoWorkerFarm {
  pub fn from_rx(
    rx: Receiver<Sender<DenoMachRequest>>,
    threads: usize,
  ) -> Self {
    let mut senders = vec![];
    let mut c = 0;

    while let Ok(sender) = rx.recv() {
      senders.push(sender);
      c += 1;
      if c == threads {
        break;
      }
    }

    Self { senders, c: 0 }
  }

  pub fn len(&self) -> usize {
    return self.senders.len();
  }

  pub fn send_all(
    &self,
    msg: DenoMachRequest,
  ) {
    for sender in self.senders.iter() {
      sender.send(msg.clone()).unwrap();
    }
  }

  pub fn send(
    &mut self,
    msg: DenoMachRequest,
  ) {
    self.senders[self.c].send(msg).unwrap();
    self.c += 1;
    if self.c > self.senders.len() - 1 {
      self.c = 0;
    }
  }
}
